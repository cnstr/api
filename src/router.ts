import * as authors from './router/search/authors.js'
import * as packages from './router/search/packages.js'
import * as repositories from './router/search/repositories.js'
import * as utility from './router/utility.js'
import { http } from './server.js'

export function load() {
	utility.load(http)
	packages.load(http)
	repositories.load(http)
	authors.load(http)
}
