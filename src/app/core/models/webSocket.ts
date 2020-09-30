import { Observable } from 'rxjs';

export interface Topic {
  name: string,
  mrid: string,
  value?: any
}

export interface UpdateMessage {
  session_id: string,
  topic: Topic
}

export interface WsMessage<T> {
  updates: UpdateMessage[]
}

export interface WebSocketConfig {
  url: string;
  reconnectInterval?: number;
  reconnectAttempts?: number;
}

export interface WebsocketService {
  //getWsData(): Observable<any>;
  sendWsData(event: string, data: any): void;
  status: Observable<boolean>;
}

export class RegisterRequest
{
  session_id: string;
  topics: Topic[]
}
