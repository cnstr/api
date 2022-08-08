import { dump, loadAll } from 'js-yaml'
import { readFile, writeFile } from 'node:fs/promises'
import { join } from 'node:path'
import { default as simple_git } from 'simple-git'

type k8s_manifest = {
	kind: string;
	spec: {
		template: {
			spec: {
				containers: Array<{
					image: string;
				}>;
			};
		};
	};
}

export async function update_k8s(tag: string) {
	const path = join('kubernetes', 'api.yaml')
	const contents = await readFile(path, 'utf8')
	const yamls = loadAll(contents) as k8s_manifest[]

	const yaml = yamls.find(y => y.kind === 'Deployment')

	if (!yaml) {
		console.log('> No deployment found to update')
		return
	}

	const { image } = yaml.spec.template.spec.containers[0]

	const name = image.split(':')[0]
	const new_name = `${name}:${tag}`

	yaml.spec.template.spec.containers[0].image = new_name
	const new_yaml = yamls.map(y => dump(y))
		.join('---\n')
	await writeFile(path, new_yaml)

	const git = simple_git('.')
	await git.commit(`chore: update k8s deployment to use ${tag}`, [path])
}
