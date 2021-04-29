// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from '@angular/core';
import { environment } from '../../../environments/environment';
import { Diagram } from '../models/diagram.model'
import { Equipment } from '../models/equipment.model';
import { Command } from '../models/command.model';
import { UpdateData } from '../models/topic.model'
import { catchError } from 'rxjs/internal/operators';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';


@Injectable({
  providedIn: 'root'
})

export class DiagramsService {
  private endpoint = environment.apiUrl;

  constructor(private httpClient: HttpClient){}    

  private handleError(error: HttpErrorResponse): any {    
    if (error.error instanceof ErrorEvent) {
      console.error('An error occurred:', error.error.message);
    } else {
      console.error(
        `Backend returned code ${error.status}, ` +
        `body was: ${error.error}`);
    }
    return throwError('An error occurred.  Check if the server is running and accessible.');    
  }

  private extractData(res: Response): any {
    const body = res;
    return body || { };
  }

  getAll() : Observable<any> {  
    
    return this.httpClient.get<Diagram>(this.endpoint + 'get-diagrams').pipe(
      catchError(this.handleError)
    );
  }

  get(id: string) : any {    
    return this.httpClient.get<Diagram>(this.endpoint + 'get-diagram?id='+id).pipe(
      catchError(this.handleError)
    );
  }

  update(diagram: Diagram) : Observable<any> {    
    console.log("Updating diagram!");        
    return this.httpClient.post<Diagram>(this.endpoint + 'save-diagram', diagram);
  }

  delete(id: string) : any {
    var diagram : Diagram = {
      diagramId: id
    };
    return this.httpClient.post<Diagram>(this.endpoint + 'delete-diagram', diagram);
  }

  create(diagram: Diagram) {
    return this.httpClient.post<Diagram>(this.endpoint + 'save-diagram', diagram).pipe(
      catchError(this.handleError)
    );  
  }

  updateData(data: UpdateData) {
    return this.httpClient.post<UpdateData>(this.endpoint + 'update-data', data).pipe(
      catchError(this.handleError)
    );  
  }

  getCommandList() : Observable<any> {
    return this.httpClient.get<Command>(this.endpoint + 'command-list').pipe(
      catchError(this.handleError)
    );
  }

  executeCommand(command: Command) {
    return this.httpClient.post<Command>(this.endpoint + 'execute-command', command).pipe(
      catchError(this.handleError)
    );  
  }

  getEquipmentList() : Observable<any> {
    return this.httpClient.get<Equipment>(this.endpoint + 'equipment-list').pipe(
      catchError(this.handleError)
    );
  }

  updateEquipment(eq: Equipment) : Observable<any> {    
    return this.httpClient.post<Equipment>(this.endpoint + 'update-equipment', eq).pipe(
      catchError(this.handleError)
    );  
  }

  deleteEquipment(id: string) : Observable<any>  {
    var user : Equipment = {
      mrid: id,
      name: ""      
    };
    return this.httpClient.post<Equipment>(this.endpoint + 'delete-equipment', user);
  }

  createEquipment(eq: Equipment) : Observable<any> {
    return this.httpClient.post<Equipment>(this.endpoint + 'create-equipment', eq).pipe(
      catchError(this.handleError)
    );      
  }
}