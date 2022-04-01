import * as authors from 'router/search/authors'
import * as packages from 'router/search/packages'
import * as repositories from 'router/search/repositories'
import * as utility from 'router/utility'
import { http } from 'server'

export function load() {
	utility.load(http)
	packages.load(http)
	repositories.load(http)
	authors.load(http)
}
