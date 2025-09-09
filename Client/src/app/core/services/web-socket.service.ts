// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import { WebSocketSubject, WebSocketSubjectConfig } from "rxjs/webSocket";
import {
  interval,
  Observable,
  Observer,
  Subject,
  SubscriptionLike,
  distinctUntilChanged,
  share,
  takeWhile,
} from "rxjs";
import {
  RegisterRequest,
  WebSocketConfig,
  WebsocketService,
  WsMessage,
} from "../models/webSocket";
import { config } from "src/app/web-socket/web-socket.config";

@Injectable({
  providedIn: "root",
})
export class WebSocketService implements WebsocketService {
  private wsConfig = inject<WebSocketConfig>(config);

  // Object configuration WebSocketSubject
  private readonly config: WebSocketSubjectConfig<WsMessage>;

  private websocketSub: SubscriptionLike;
  private statusSub: SubscriptionLike;

  // Observable for reconnect by interval
  private reconnection$: Observable<number>;
  private websocket$: WebSocketSubject<WsMessage>;

  // Reports when a connection and reconnect occurs
  private connected$: Observer<boolean>;
  public wsConnection$: Subject<boolean>;

  // Helper Observable for working with message subscriptions
  public wsMessages$: Subject<WsMessage>;

  // Pause between reconnection attempts in milliseconds
  private reconnectInterval: number;

  // Number of reconnect attempts
  private reconnectAttempts: number;

  // Synchronous helper for connection status
  private isConnected: boolean;

  private disconnectRequested: boolean = false;

  // Connection status
  public status: Observable<boolean>;

  constructor() {
    const wsConfig = this.wsConfig;

    this.wsMessages$ = new Subject<WsMessage>();
    this.wsConnection$ = new Subject<boolean>();

    this.reconnectInterval = wsConfig.reconnectInterval || 5000;
    this.reconnectAttempts = wsConfig.reconnectAttempts || 10;

    this.config = {
      url: wsConfig.url,
      closeObserver: {
        next: () => {
          this.connected$.next(false);
        },
      },
      openObserver: {
        next: () => {
          this.connected$.next(true);
        },
      },
    };

    this.status = new Observable<boolean>((observer) => {
      this.connected$ = observer;
    }).pipe(share(), distinctUntilChanged());

    this.statusSub = this.status.subscribe({
      next: (isConnected) => {
        console.log("Connection status has changed: Connected=" + isConnected);
        this.isConnected = isConnected;
        this.wsConnection$.next(isConnected);

        if (
          !this.disconnectRequested &&
          !this.reconnection$ &&
          typeof isConnected === "boolean" &&
          !isConnected
        ) {
          console.log("Lost connection to WS server.  Reconnect...");
          this.reconnect();
        } else {
          this.statusSub.unsubscribe();
        }
      },
    });
  }

  // Makes WebSocket connection
  public connect(sessionId: string) {
    this.config.url = this.wsConfig.url + sessionId;
    this.websocket$ = new WebSocketSubject(this.config);
    this.websocket$.subscribe({
      next: (message) => {
        this.wsMessages$.next(message);
      },
      error: (error: Event) => {
        console.error(error);
        if (!this.websocket$) {
          this.reconnect();
        }
      },
    });
  }

  public disconnect() {
    this.disconnectRequested = true;
    this.websocket$?.complete();
  }

  // Makes WebSocket reconnection
  private reconnect(): void {
    this.reconnection$ = interval(this.reconnectInterval).pipe(
      takeWhile(() => !this.websocket$),
    );

    this.reconnection$.subscribe({
      next: () => {
        this.connect(this.config.url);
      },
      complete: () => {
        this.reconnection$ = null;

        if (!this.websocket$) {
          this.wsMessages$.complete();
          this.connected$.complete();
        }
      },
    });
  }

  // Sends WebSocket message
  public sendWsData(data: RegisterRequest): void {
    if (this.isConnected) {
      this.websocket$.next(<any>data);
    }
  }

  // get connection status
  public getConnectionStatus(): boolean {
    return this.isConnected;
  }
}
