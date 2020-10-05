import { Inject, Injectable, OnDestroy } from '@angular/core';
import { WebSocketSubject, WebSocketSubjectConfig } from 'rxjs/webSocket';
import { interval, Observable, Observer, Subject, SubscriptionLike } from 'rxjs';
import { WebSocketConfig, WebsocketService, WsMessage } from '../models/webSocket';
import { config } from 'src/app/web-socket/web-socket.config';
import { distinctUntilChanged, map, share, takeWhile } from 'rxjs/operators';


@Injectable({
  providedIn: 'root'
})
export class WebSocketService implements WebsocketService, OnDestroy {

  // Object configuration WebSocketSubject
  private readonly config: WebSocketSubjectConfig<WsMessage<any>>;

  private websocketSub: SubscriptionLike;
  private statusSub: SubscriptionLike;

  // Observable for reconnect by interval
  private reconnection$: Observable<number>;
  private websocket$: WebSocketSubject<WsMessage<any>>;

  // Reports when a connection and reconnect occurs
  private connected$: Observer<boolean>;
  public wsConnection$: Subject<boolean>;

  // Helper Observable for working with message subscriptions
  public wsMessages$: Subject<WsMessage<any>>;  

  // Pause between reconnection attempts in milliseconds
  private reconnectInterval: number;

  // Number of reconnect attempts
  private reconnectAttempts: number;

  // Synchronous helper for connection status
  private isConnected: boolean;

  // Connection status
  public status: Observable<boolean>;

  constructor(@Inject(config) private wsConfig: WebSocketConfig) {
    this.wsMessages$ = new Subject<WsMessage<any>>();
    this.wsConnection$ = new Subject<boolean>();

    this.reconnectInterval = wsConfig.reconnectInterval || 5000;
    this.reconnectAttempts = wsConfig.reconnectAttempts || 10;

    this.config = {
      url: wsConfig.url,
      closeObserver: {
        next: () => {
          this.websocket$ = null;
          this.connected$.next(false);
        }
      },
      openObserver: {
        next: () => {
          console.log('WebSocket connected!');
          this.connected$.next(true);          
        }
      }
    };

    this.status = new Observable<boolean>((observer) => {
      this.connected$ = observer;
    }).pipe(share(), distinctUntilChanged());

    this.statusSub = this.status
      .subscribe((isConnected) => {
        console.log("Connection status has changed: Connected=" + isConnected);
        this.isConnected = isConnected;
        this.wsConnection$.next(isConnected);
        if (!this.reconnection$ && typeof(isConnected) === 'boolean' && !isConnected) {
          console.log("Lost connection to WS server.  Reconnect...");
          this.reconnect();
        }
      });    
  }

  // Makes WebSocket connection
  public connect(sessionId: string) {
    this.config.url = this.wsConfig.url + sessionId;
    this.websocket$ = new WebSocketSubject(this.config);    
    this.websocket$.subscribe(
      (message) => {        
        this.wsMessages$.next(message);
      },
      (error: Event) => {
        if (!this.websocket$) {
          this.reconnect();
        }
      });    
  }

  // Makes WebSocket reconnection
  private reconnect(): void {
    this.reconnection$ = interval(this.reconnectInterval)
      .pipe(takeWhile((v, index) => index < this.reconnectAttempts && !this.websocket$));

    this.reconnection$.subscribe(
      () => this.connect(this.config.url),
      null,
      () => {
        this.reconnection$ = null;

        if (!this.websocket$) {
          this.wsMessages$.complete();
          this.connected$.complete();
        }
      });
  }

  // GET WebSocket data
  //getWsData(): Observable<any> {
      //return this.wsMessages$.asObservable().pipe(map(data => JSON.parse(data)));
  //}

  // Sends WebSocket message
  sendWsData(data: any = {}): void {
    if (this.isConnected) {
      var json = JSON.stringify(data);
      //console.log("sendWsData: " + json);
      this.websocket$.next(<any>data);
    } else {
      console.error('Unable to send message.  The websocket is not connected.');
    }
  }

  ngOnDestroy() {
    this.websocketSub.unsubscribe();
    this.statusSub.unsubscribe();
  }
}
