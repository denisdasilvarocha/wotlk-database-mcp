// Exports the milkyway-codex data arrays as one JSON object. Usage: bun export.ts <checkout dir>
import { resolve } from 'node:path'

const data = (file: string) => import(resolve(Bun.argv[2], 'src/data', `${file}.ts`))
const fns = await data('api-functions'), ev = await data('events'), wd = await data('widgets'),
  cv = await data('cvars'), ty = await data('data-types'), cl = await data('combat-log-events'),
  st = await data('secure-templates')
console.log(JSON.stringify({
  functions: fns.API_FUNCTIONS, events: ev.EVENTS, widgets: wd.WIDGETS, cvars: cv.CVARS, types: ty.DATA_TYPES,
  combatlog_prefixes: cl.COMBAT_LOG_PREFIXES, combatlog_events: cl.COMBAT_LOG_SUB_EVENTS,
  secure_templates: st.SECURE_TEMPLATES,
}, null, 1))
