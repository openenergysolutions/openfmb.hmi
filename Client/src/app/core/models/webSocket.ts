// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Observable } from "rxjs";
import { Topic } from "../../shared/models/topic.model";

export interface UpdateMessage {
  profile?: string;
  session_id: string;
  topic: Topic;
}

export interface WsMessage {
  updates: UpdateMessage[];
}

export interface WebSocketConfig {
  url: string;
  reconnectInterval?: number;
  reconnectAttempts?: number;
}

export interface WebsocketService {
  //getWsData(): Observable<any>;
  sendWsData(data: RegisterRequest): void;
  status: Observable<boolean>;
}

export class RegisterRequest {
  session_id: string;
  topics: Topic[];
}
