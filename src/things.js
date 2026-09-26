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

const t = Application("Things3");

function run(argv) {
  const req = JSON.parse(argv[0]);
  const ops = { todos, search, get, projects, areas, tags, add, addProject, update, remove, show };
  return JSON.stringify(ops[req.op](req));
}

function applescript(src) {
  const app = Application.currentApplication();
  app.includeStandardAdditions = true;
  return app.runScript(`tell application "Things3"\n${src}\nend tell`, { in: "AppleScript" });
}

function formatDate(d) {
  if (!d) return null;
  const pad = n => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

function parseDate(s) {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(s);
  if (!m) throw new Error(`expected yyyy-mm-dd, got ${s}`);
  return new Date(+m[1], +m[2] - 1, +m[3]);
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

function item(id) {
  const spec = t.toDos.byId(id);
  if (!exists(spec)) throw new Error(`no to-do or project with id ${id}`);
  return spec;
}

function columns(c) {
  const id = c.id(), title = c.name(), status = c.status(), notes = c.notes(), when = c.activationDate(),
    deadline = c.dueDate(), tags = c.tagNames(), project = c.project.name(), projectId = c.project.id(),
    area = c.area.name(), areaId = c.area.id();
  return id.map((_, i) => ({
    id: id[i], title: title[i], status: status[i], notes: notes[i] || null, when: formatDate(when[i]),
    deadline: formatDate(deadline[i]), tags: tagList(tags[i]), project: project[i], projectId: projectId[i],
    area: area[i], areaId: areaId[i],
  }));
}

function page(items, limit) {
  return { total: items.length, items: items.slice(0, limit) };
}

function todos(req) {
  let src;
  if (req.list) src = t.lists.byId(LISTS[req.list]);
  else if (req.project) src = find(t.projects, "project", req.project);
  else if (req.area) src = find(t.areas, "area", req.area);
  else src = find(t.tags, "tag", req.tag);
  return page(columns(src.toDos), req.limit);
}

function search(req) {
  const lists = ["inbox", "today", "anytime", "upcoming", "someday"];
  if (req.includeCompleted) lists.push("logbook");
  const q = req.query;
  const seen = new Set();
  const items = [];
  for (const l of lists) {
    const c = t.lists.byId(LISTS[l]).toDos.whose({ _or: [{ name: { _contains: q } }, { notes: { _contains: q } }] });
    for (const it of columns(c)) {
      if (seen.has(it.id)) continue;
      seen.add(it.id);
      items.push(it);
    }
  }
  return page(items, req.limit);
}

function get(req) {
  const project = t.projects.byId(req.id);
  const isProject = exists(project);
  const p = item(req.id).properties();
  const out = {
    id: p.id, type: isProject ? "project" : "to-do", title: p.name, status: p.status,
    notes: p.notes || null, when: formatDate(p.activationDate), deadline: formatDate(p.dueDate),
    tags: tagList(p.tagNames), project: p.project ? p.project.name() : null,
    projectId: p.project ? p.project.id() : null, area: p.area ? p.area.name() : null,
    areaId: p.area ? p.area.id() : null, created: p.creationDate, modified: p.modificationDate,
    completed: p.completionDate, canceled: p.cancellationDate,
  };
  if (isProject) {
    const c = project.toDos;
    const name = c.name(), status = c.status();
    out.toDos = c.id().map((id, i) => ({ id, title: name[i], status: status[i] }));
  }
  return out;
}

function projects(req) {
  const c = t.projects;
  const id = c.id(), name = c.name(), status = c.status(), notes = c.notes(), when = c.activationDate(),
    deadline = c.dueDate(), tags = c.tagNames(), area = c.area.name(), areaId = c.area.id();
  const areaFilter = req.area ? find(t.areas, "area", req.area).id() : null;
  return id.map((_, i) => ({
    id: id[i], title: name[i], status: status[i], notes: notes[i] || null, when: formatDate(when[i]),
    deadline: formatDate(deadline[i]), tags: tagList(tags[i]), area: area[i], areaId: areaId[i],
  })).filter(p => !areaFilter || p.areaId === areaFilter);
}

function areas() {
  const c = t.areas;
  const id = c.id(), name = c.name(), tags = c.tagNames();
  return id.map((_, i) => ({ id: id[i], title: name[i], tags: tagList(tags[i]) }));
}

function tags() {
  const c = t.tags;
  const id = c.id(), name = c.name(), parent = c.parentTag.name();
  return id.map((_, i) => ({ id: id[i], title: name[i], parent: parent[i] }));
}

function setWhen(id, when) {
  const ref = `to do id "${id}"`;
  const move = list => applescript(`move ${ref} to list id "${LISTS[list]}"`);
  if (when === "today" || when === "anytime" || when === "someday") return move(when);
  if (when === "tomorrow") {
    const d = new Date();
    d.setDate(d.getDate() + 1);
    return t.schedule(item(id), { for: new Date(d.getFullYear(), d.getMonth(), d.getDate()) });
  }
  if (!/^\d{4}-\d{2}-\d{2}$/.test(when)) throw new Error("when must be today, tomorrow, anytime, someday, or yyyy-mm-dd");
  t.schedule(item(id), { for: parseDate(when) });
}

function apply(id, req) {
  const it = item(id);
  const ref = `to do id "${id}"`;
  if (req.title !== undefined) it.name = req.title;
  if (req.notes !== undefined) it.notes = req.notes;
  if (req.appendNotes) it.notes = [it.notes(), req.appendNotes].filter(Boolean).join("\n");
  if (req.tags !== undefined) it.tagNames = req.tags.join(", ");
  if (req.addTags) it.tagNames = [...new Set([...tagList(it.tagNames()), ...req.addTags])].join(", ");
  if (req.deadline === "") applescript(`delete due date of ${ref}`);
  else if (req.deadline !== undefined) it.dueDate = parseDate(req.deadline);
  if (req.project === "") applescript(`delete project of ${ref}`);
  else if (req.project !== undefined) it.project = find(t.projects, "project", req.project);
  if (req.area === "") applescript(`delete area of ${ref}`);
  else if (req.area !== undefined) it.area = find(t.areas, "area", req.area);
  if (req.when !== undefined) setWhen(id, req.when);
  if (req.status !== undefined) it.status = req.status;
  return get({ id });
}

function add(req) {
  const td = t.ToDo({ name: req.title });
  t.toDos.push(td);
  return apply(td.id(), { ...req, title: undefined });
}

function addProject(req) {
  const p = t.Project({ name: req.title });
  t.projects.push(p);
  const id = p.id();
  for (const title of req.toDos || []) {
    const td = t.ToDo({ name: title });
    t.toDos.push(td);
    td.project = t.projects.byId(id);
  }
  return apply(id, { ...req, title: undefined });
}

function update(req) {
  return apply(req.id, req);
}

function remove(req) {
  const out = get(req);
  t.delete(out.type === "project" ? t.projects.byId(req.id) : item(req.id));
  return { deleted: out.id, title: out.title };
}

function show(req) {
  let target;
  if (req.list) target = t.lists.byId(LISTS[req.list]);
  else if (exists(t.toDos.byId(req.id))) target = t.toDos.byId(req.id);
  else if (exists(t.areas.byId(req.id))) target = t.areas.byId(req.id);
  else target = find(t.tags, "item", req.id);
  t.show(target);
  return { shown: req.list || req.id };
}
