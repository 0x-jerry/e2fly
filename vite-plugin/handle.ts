import { spawn, ChildProcess } from 'child_process'
import pc from 'picocolors'
import { build, BuildOptions } from 'esbuild'
import electron from 'electron'
import { build as electronBuilder } from 'electron-builder'
import { ElectronEntry, ResolvedViteElectronBuilderOptions, AssetsEntry } from './types'
import fs from 'fs-extra'

const toArray = <T>(o: T[] | T): T[] => (Array.isArray(o) ? o : [o])

function runMainProcess(mainFile: string) {
  return spawn(electron as any, [mainFile], { stdio: 'inherit' })
}

export async function handleDev(options: ResolvedViteElectronBuilderOptions) {
  const esbuildOptions = createEsbuildOptions(options)

  for (const entry of toArray(options.entry)) {
    await buildWithESBuild(esbuildOptions, entry, true)
  }

  await copyAssets(toArray(options.assetsDir))
}

export async function handleBuild(options: ResolvedViteElectronBuilderOptions) {
  const { electronBuilderConfig } = options
  const esbuildOptions = createEsbuildOptions(options)

  try {
    for (const entry of toArray(options.entry)) {
      await buildWithESBuild(esbuildOptions, entry, false)
    }

    await copyAssets(toArray(options.assetsDir))

    await electronBuilder({
      config: electronBuilderConfig,
    })

    console.log(pc.green('Main Process Build Succeeded.'))
  } catch (error) {
    console.log(`\n${pc.red('Main Process Build Failed')}\n`, error, '\n')
  }
}

async function buildWithESBuild(esbuildOpt: BuildOptions, entry: ElectronEntry, isDev: boolean) {
  let child: ChildProcess

  await build({
    ...esbuildOpt,
    entryPoints: [entry.input],
    outfile: entry.output,
    watch: isDev
      ? {
          onRebuild(error) {
            if (!entry.electron) return

            if (error) {
              console.error(pc.red('Rebuild Failed:'), error)
            } else {
              console.log(pc.green('Rebuild Succeeded'))
              if (child) child.kill()
              child = runMainProcess(entry.output)
            }
          },
        }
      : false,
  })

  if (isDev && entry.electron) {
    child = runMainProcess(entry.output)
    console.log(pc.bgGreen(pc.white(` ⚡Main Process Running `)))
  }
}

function createEsbuildOptions(options: ResolvedViteElectronBuilderOptions): BuildOptions {
  const define = Object.entries(options.env).reduce((preVal, [key, value]) => {
    preVal[`process.env.${key}`] = JSON.stringify(value)
    return preVal
  }, {})

  const { tsconfig, external } = options

  return {
    target: 'es2020',
    format: 'cjs',
    bundle: true,
    platform: 'node',
    define,
    tsconfig,
    external,
  }
}

async function copyAssets(dirs: AssetsEntry[]) {
  for (const dir of dirs) {
    await fs.copy(dir.input, dir.output, {
      overwrite: false,
      errorOnExist: false,
      recursive: true,
    })
  }
}
