import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { StatsServiceClient as _v2ray_core_app_stats_command_StatsServiceClient, StatsServiceDefinition as _v2ray_core_app_stats_command_StatsServiceDefinition } from './v2ray/core/app/stats/command/StatsService';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  v2ray: {
    core: {
      app: {
        stats: {
          command: {
            Config: MessageTypeDefinition
            GetStatsRequest: MessageTypeDefinition
            GetStatsResponse: MessageTypeDefinition
            QueryStatsRequest: MessageTypeDefinition
            QueryStatsResponse: MessageTypeDefinition
            Stat: MessageTypeDefinition
            StatsService: SubtypeConstructor<typeof grpc.Client, _v2ray_core_app_stats_command_StatsServiceClient> & { service: _v2ray_core_app_stats_command_StatsServiceDefinition }
            SysStatsRequest: MessageTypeDefinition
            SysStatsResponse: MessageTypeDefinition
          }
        }
      }
    }
  }
}

