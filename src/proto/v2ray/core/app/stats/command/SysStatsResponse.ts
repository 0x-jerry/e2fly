// Original file: grpc/v2ray.proto

import type { Long } from '@grpc/proto-loader';

export interface SysStatsResponse {
  'NumGoroutine'?: (number);
  'NumGC'?: (number);
  'Alloc'?: (number | string | Long);
  'TotalAlloc'?: (number | string | Long);
  'Sys'?: (number | string | Long);
  'Mallocs'?: (number | string | Long);
  'Frees'?: (number | string | Long);
  'LiveObjects'?: (number | string | Long);
  'PauseTotalNs'?: (number | string | Long);
  'Uptime'?: (number);
}

export interface SysStatsResponse__Output {
  'NumGoroutine': (number);
  'NumGC': (number);
  'Alloc': (string);
  'TotalAlloc': (string);
  'Sys': (string);
  'Mallocs': (string);
  'Frees': (string);
  'LiveObjects': (string);
  'PauseTotalNs': (string);
  'Uptime': (number);
}
