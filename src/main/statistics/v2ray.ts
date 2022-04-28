import * as protoLoader from '@grpc/proto-loader'
import * as grpc from '@grpc/grpc-js'
import path from 'path'
import { ProtoGrpcType } from '@/proto/v2ray'

const protoFileName = path.resolve('grpc', 'v2ray.proto')

const packageDefinition = protoLoader.loadSync(protoFileName, {})
const pkg = grpc.loadPackageDefinition(packageDefinition) as any as ProtoGrpcType

const StatsService = pkg.v2ray.core.app.stats.command.StatsService

// todo: change host at runtime
const host = 'localhost:10086'
const client = new StatsService(host, grpc.credentials.createInsecure())

client.queryStats(
  {
    pattern: '',
    reset: false,
  },
  (err, val) => {
    if (err) {
      console.error(err)
      return
    }

    console.log(val?.stat)
  }
)
