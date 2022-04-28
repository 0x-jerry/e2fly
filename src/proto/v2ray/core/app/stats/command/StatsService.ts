// Original file: grpc/v2ray.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { GetStatsRequest as _v2ray_core_app_stats_command_GetStatsRequest, GetStatsRequest__Output as _v2ray_core_app_stats_command_GetStatsRequest__Output } from '../../../../../v2ray/core/app/stats/command/GetStatsRequest';
import type { GetStatsResponse as _v2ray_core_app_stats_command_GetStatsResponse, GetStatsResponse__Output as _v2ray_core_app_stats_command_GetStatsResponse__Output } from '../../../../../v2ray/core/app/stats/command/GetStatsResponse';
import type { QueryStatsRequest as _v2ray_core_app_stats_command_QueryStatsRequest, QueryStatsRequest__Output as _v2ray_core_app_stats_command_QueryStatsRequest__Output } from '../../../../../v2ray/core/app/stats/command/QueryStatsRequest';
import type { QueryStatsResponse as _v2ray_core_app_stats_command_QueryStatsResponse, QueryStatsResponse__Output as _v2ray_core_app_stats_command_QueryStatsResponse__Output } from '../../../../../v2ray/core/app/stats/command/QueryStatsResponse';
import type { SysStatsRequest as _v2ray_core_app_stats_command_SysStatsRequest, SysStatsRequest__Output as _v2ray_core_app_stats_command_SysStatsRequest__Output } from '../../../../../v2ray/core/app/stats/command/SysStatsRequest';
import type { SysStatsResponse as _v2ray_core_app_stats_command_SysStatsResponse, SysStatsResponse__Output as _v2ray_core_app_stats_command_SysStatsResponse__Output } from '../../../../../v2ray/core/app/stats/command/SysStatsResponse';

export interface StatsServiceClient extends grpc.Client {
  GetStats(argument: _v2ray_core_app_stats_command_GetStatsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  GetStats(argument: _v2ray_core_app_stats_command_GetStatsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_v2ray_core_app_stats_command_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  GetStats(argument: _v2ray_core_app_stats_command_GetStatsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  GetStats(argument: _v2ray_core_app_stats_command_GetStatsRequest, callback: grpc.requestCallback<_v2ray_core_app_stats_command_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  getStats(argument: _v2ray_core_app_stats_command_GetStatsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  getStats(argument: _v2ray_core_app_stats_command_GetStatsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_v2ray_core_app_stats_command_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  getStats(argument: _v2ray_core_app_stats_command_GetStatsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  getStats(argument: _v2ray_core_app_stats_command_GetStatsRequest, callback: grpc.requestCallback<_v2ray_core_app_stats_command_GetStatsResponse__Output>): grpc.ClientUnaryCall;
  
  GetSysStats(argument: _v2ray_core_app_stats_command_SysStatsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_SysStatsResponse__Output>): grpc.ClientUnaryCall;
  GetSysStats(argument: _v2ray_core_app_stats_command_SysStatsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_v2ray_core_app_stats_command_SysStatsResponse__Output>): grpc.ClientUnaryCall;
  GetSysStats(argument: _v2ray_core_app_stats_command_SysStatsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_SysStatsResponse__Output>): grpc.ClientUnaryCall;
  GetSysStats(argument: _v2ray_core_app_stats_command_SysStatsRequest, callback: grpc.requestCallback<_v2ray_core_app_stats_command_SysStatsResponse__Output>): grpc.ClientUnaryCall;
  getSysStats(argument: _v2ray_core_app_stats_command_SysStatsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_SysStatsResponse__Output>): grpc.ClientUnaryCall;
  getSysStats(argument: _v2ray_core_app_stats_command_SysStatsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_v2ray_core_app_stats_command_SysStatsResponse__Output>): grpc.ClientUnaryCall;
  getSysStats(argument: _v2ray_core_app_stats_command_SysStatsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_SysStatsResponse__Output>): grpc.ClientUnaryCall;
  getSysStats(argument: _v2ray_core_app_stats_command_SysStatsRequest, callback: grpc.requestCallback<_v2ray_core_app_stats_command_SysStatsResponse__Output>): grpc.ClientUnaryCall;
  
  QueryStats(argument: _v2ray_core_app_stats_command_QueryStatsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_QueryStatsResponse__Output>): grpc.ClientUnaryCall;
  QueryStats(argument: _v2ray_core_app_stats_command_QueryStatsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_v2ray_core_app_stats_command_QueryStatsResponse__Output>): grpc.ClientUnaryCall;
  QueryStats(argument: _v2ray_core_app_stats_command_QueryStatsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_QueryStatsResponse__Output>): grpc.ClientUnaryCall;
  QueryStats(argument: _v2ray_core_app_stats_command_QueryStatsRequest, callback: grpc.requestCallback<_v2ray_core_app_stats_command_QueryStatsResponse__Output>): grpc.ClientUnaryCall;
  queryStats(argument: _v2ray_core_app_stats_command_QueryStatsRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_QueryStatsResponse__Output>): grpc.ClientUnaryCall;
  queryStats(argument: _v2ray_core_app_stats_command_QueryStatsRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_v2ray_core_app_stats_command_QueryStatsResponse__Output>): grpc.ClientUnaryCall;
  queryStats(argument: _v2ray_core_app_stats_command_QueryStatsRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_v2ray_core_app_stats_command_QueryStatsResponse__Output>): grpc.ClientUnaryCall;
  queryStats(argument: _v2ray_core_app_stats_command_QueryStatsRequest, callback: grpc.requestCallback<_v2ray_core_app_stats_command_QueryStatsResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface StatsServiceHandlers extends grpc.UntypedServiceImplementation {
  GetStats: grpc.handleUnaryCall<_v2ray_core_app_stats_command_GetStatsRequest__Output, _v2ray_core_app_stats_command_GetStatsResponse>;
  
  GetSysStats: grpc.handleUnaryCall<_v2ray_core_app_stats_command_SysStatsRequest__Output, _v2ray_core_app_stats_command_SysStatsResponse>;
  
  QueryStats: grpc.handleUnaryCall<_v2ray_core_app_stats_command_QueryStatsRequest__Output, _v2ray_core_app_stats_command_QueryStatsResponse>;
  
}

export interface StatsServiceDefinition extends grpc.ServiceDefinition {
  GetStats: MethodDefinition<_v2ray_core_app_stats_command_GetStatsRequest, _v2ray_core_app_stats_command_GetStatsResponse, _v2ray_core_app_stats_command_GetStatsRequest__Output, _v2ray_core_app_stats_command_GetStatsResponse__Output>
  GetSysStats: MethodDefinition<_v2ray_core_app_stats_command_SysStatsRequest, _v2ray_core_app_stats_command_SysStatsResponse, _v2ray_core_app_stats_command_SysStatsRequest__Output, _v2ray_core_app_stats_command_SysStatsResponse__Output>
  QueryStats: MethodDefinition<_v2ray_core_app_stats_command_QueryStatsRequest, _v2ray_core_app_stats_command_QueryStatsResponse, _v2ray_core_app_stats_command_QueryStatsRequest__Output, _v2ray_core_app_stats_command_QueryStatsResponse__Output>
}
