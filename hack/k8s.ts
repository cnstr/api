import { readFile, writeFile } from 'node:fs/promises'
import { join } from 'node:path'

import { dump, loadAll } from 'js-yaml'
import { simpleGit } from 'simple-git'

type KubernetesManifest = {
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

export async function bumpDeployManifest(tag: string) {
	const path = join('kubernetes', 'api.yaml')
	const contents = await readFile(path, 'utf8')
	const yamls = loadAll(contents) as KubernetesManifest[]

	const yaml = yamls.find(y => y.kind === 'Deployment')

	if (!yaml) {
		console.log('> No deployment found to update')
		return
	}

	const { image } = yaml.spec.template.spec.containers[0]

	const name = image.split(':')[0]
	const bumpedTagName = `${name}:${tag}`

	yaml.spec.template.spec.containers[0].image = bumpedTagName
	const new_yaml = yamls.map(y => dump(y))
		.join('---\n')
	await writeFile(path, new_yaml)

	const git = simpleGit('.')
	await git.commit(`chore: bump k8s -> ${tag}`, [path])
	await git.push()
}
