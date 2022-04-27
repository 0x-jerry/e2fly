declare global {
  namespace NodeJS {
    interface ProcessEnv {
      /**
       * WS address
       */
      E2FLY_WS_ADDR?: string
      /**
       * RPC protocol ID
       */
      E2FLY_RPC_PROTOCOL_ID?: string
    }
  }
}

export {}
