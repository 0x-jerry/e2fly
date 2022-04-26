import { spawn, ChildProcess } from 'child_process'
import pc from 'picocolors'
import { build } from 'esbuild'
import electron from 'electron'
import { createEsbuildOptions } from './esbuild.options'
import { build as electronBuilder } from 'electron-builder'
import { ResolvedViteElectronBuilderOptions } from './types'

function runMainProcess(mainFile: string) {
  return spawn(electron as any, [mainFile], { stdio: 'inherit' })
}

export async function handleDev(options: ResolvedViteElectronBuilderOptions) {
  const { outputFile: mainFile } = options
  const esbuildOptions = createEsbuildOptions(options)

  let child: ChildProcess

  await build({
    ...esbuildOptions,
    watch: {
      onRebuild(error) {
        if (error) {
          console.error(pc.red('Rebuild Failed:'), error)
        } else {
          console.log(pc.green('Rebuild Succeeded'))
          if (child) child.kill()
          child = runMainProcess(mainFile)
        }
      },
    },
  })

  console.log(pc.bgGreen(pc.white(` ⚡Main Process Running `)))

  child = runMainProcess(mainFile)
}

export async function handleBuild(options: ResolvedViteElectronBuilderOptions) {
  const { electronBuilderConfig } = options
  const esbuildOptions = createEsbuildOptions(options)

  try {
    await build(esbuildOptions)
    await options.afterEsbuildBuild()

    await electronBuilder({
      config: electronBuilderConfig,
    })

    console.log(pc.green('Main Process Build Succeeded.'))
  } catch (error) {
    console.log(`\n${pc.red('Main Process Build Failed')}\n`, error, '\n')
  }
}
