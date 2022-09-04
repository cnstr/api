import { Landing } from './schemas/landing.js'
import { Package } from './schemas/package.js'
import { Repository } from './schemas/repository.js'
import { BadRequest, NotFoundRequest } from './schemas/requests.js'

export const schemas = {
	BadRequest,
	NotFoundRequest,
	Package,
	Repository,
	Landing
}
