export interface AppConfig {
    proxy:            Proxy;
    activeOutboundId: string;
    v2fly:            V2Fly;
    outbound:         Outbound[];
}

export interface Outbound {
    id:     string;
    label:  string;
    config: string;
}

export interface Proxy {
    system: boolean;
    pac:    boolean;
}

export interface V2Fly {
    bin:    string;
    http:   HTTP;
    socks:  HTTP;
    stream: Stream;
    routes: Routes;
}

export interface HTTP {
    enabled: boolean;
    address: string;
    port:    number;
}

export interface Routes {
    bypassCN: boolean;
    blockAds: boolean;
}

export interface Stream {
    udp: boolean;
    tcp: boolean;
}
