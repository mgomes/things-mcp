const LISTS = {
  inbox: "TMInboxListSource",
  today: "TMTodayListSource",
  tomorrow: "tomorrow",
  anytime: "TMNextListSource",
  upcoming: "TMCalendarListSource",
  someday: "TMSomedayListSource",
  logbook: "TMLogbookListSource",
  trash: "TMTrashListSource",
};

function run(argv) {
  const req = JSON.parse(argv[0]);
  const t = Application("Things3");
  const ops = { todos, get, projects, areas, tags };
  return JSON.stringify(ops[req.op](t, req));
}

function date(d) {
  if (!d) return null;
  const pad = n => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

function tagList(s) {
  return s ? s.split(", ") : [];
}

function exists(spec) {
  try { spec.id(); return true } catch (_) { return false }
}

function find(coll, kind, key) {
  for (const spec of [coll.byId(key), coll.byName(key)]) if (exists(spec)) return spec;
  throw new Error(`${kind} not found: ${key}`);
}

function source(t, req) {
  if (req.list) {
    const id = LISTS[req.list];
    if (!id) throw new Error(`unknown list: ${req.list}`);
    return t.lists.byId(id);
  }
  if (req.project) return find(t.projects, "project", req.project);
  if (req.area) return find(t.areas, "area", req.area);
  if (req.tag) return find(t.tags, "tag", req.tag);
  throw new Error("provide list, project, area, or tag");
}

function todos(t, req) {
  const c = source(t, req).toDos;
  const cols = {
    id: c.id(), title: c.name(), status: c.status(), notes: c.notes(),
    when: c.activationDate(), deadline: c.dueDate(), tags: c.tagNames(),
    project: c.project.name(), projectId: c.project.id(),
    area: c.area.name(), areaId: c.area.id(),
  };
  const total = cols.id.length;
  const items = [];
  for (let i = 0; i < Math.min(total, req.limit); i++) {
    items.push({
      id: cols.id[i], title: cols.title[i], status: cols.status[i], notes: cols.notes[i] || null,
      when: date(cols.when[i]), deadline: date(cols.deadline[i]), tags: tagList(cols.tags[i]),
      project: cols.project[i], projectId: cols.projectId[i], area: cols.area[i], areaId: cols.areaId[i],
    });
  }
  return { total, items };
}

function get(t, req) {
  const project = t.projects.byId(req.id);
  const isProject = exists(project);
  let p;
  try { p = (isProject ? project : t.toDos.byId(req.id)).properties() } catch (_) {
    throw new Error(`no to-do or project with id ${req.id}`);
  }
  const out = {
    id: p.id, type: isProject ? "project" : "to-do", title: p.name, status: p.status,
    notes: p.notes || null, when: date(p.activationDate), deadline: date(p.dueDate), tags: tagList(p.tagNames),
    project: p.project ? p.project.name() : null, projectId: p.project ? p.project.id() : null,
    area: p.area ? p.area.name() : null, areaId: p.area ? p.area.id() : null,
    created: p.creationDate, modified: p.modificationDate, completed: p.completionDate, canceled: p.cancellationDate,
  };
  if (isProject) {
    const c = project.toDos;
    const name = c.name(), status = c.status();
    out.toDos = c.id().map((id, i) => ({ id, title: name[i], status: status[i] }));
  }
  return out;
}

function projects(t, req) {
  const c = t.projects;
  const id = c.id(), name = c.name(), status = c.status(), notes = c.notes(), when = c.activationDate(),
    deadline = c.dueDate(), tags = c.tagNames(), area = c.area.name(), areaId = c.area.id();
  const areaFilter = req.area ? find(t.areas, "area", req.area).id() : null;
  return id.map((_, i) => ({
    id: id[i], title: name[i], status: status[i], notes: notes[i] || null, when: date(when[i]),
    deadline: date(deadline[i]), tags: tagList(tags[i]), area: area[i], areaId: areaId[i],
  })).filter(p => !areaFilter || p.areaId === areaFilter);
}

function areas(t) {
  const c = t.areas;
  const id = c.id(), name = c.name(), tags = c.tagNames();
  return id.map((_, i) => ({ id: id[i], title: name[i], tags: tagList(tags[i]) }));
}

function tags(t) {
  const c = t.tags;
  const id = c.id(), name = c.name(), parent = c.parentTag.name();
  return id.map((_, i) => ({ id: id[i], title: name[i], parent: parent[i] }));
}
