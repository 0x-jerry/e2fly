declare global {
  namespace NodeJS {
    interface ProcessEnv {
      DEV_SERVER_URL?: string
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
