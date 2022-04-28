// Original file: grpc/v2ray.proto

import type { Long } from '@grpc/proto-loader';

export interface Stat {
  'name'?: (string);
  'value'?: (number | string | Long);
}

export interface Stat__Output {
  'name': (string);
  'value': (string);
}
