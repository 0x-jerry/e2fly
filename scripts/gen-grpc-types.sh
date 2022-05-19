# https://github.com/grpc/grpc-node/tree/master/packages/proto-loader#generating-typescript-types

# clean src/proto
[ -d src/proto ] && rm -rf src/proto

$(npm bin)/proto-loader-gen-types --longs=String --enums=String --defaults --oneofs --grpcLib=@grpc/grpc-js --outDir=src/proto/ grpc/v2ray.proto
