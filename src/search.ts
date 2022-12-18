import { Client } from '@elastic/elasticsearch'

export const elastic = new Client({
	node: 'http://search:9200'
})
