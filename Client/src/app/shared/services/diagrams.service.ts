import { Injectable } from '@angular/core';
import { config } from '../../../config'
import { Diagram } from '../models/diagram.model'
import { UpdateData } from '../models/topic.model'
import { catchError } from 'rxjs/internal/operators';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';


@Injectable({
  providedIn: 'root'
})

export class DiagramsService {
  private endpoint = config.apiUrl;

  constructor(private httpClient: HttpClient) { }      

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
    
    return this.httpClient.get<Diagram>(this.endpoint + 'diagrams').pipe(
      catchError(this.handleError)
    );
  }

  get(id: string) : any {    
    return this.httpClient.get<Diagram>(this.endpoint + 'diagram?id='+id).pipe(
      catchError(this.handleError)
    );
  }

  update(diagram: Diagram) : Observable<any> {    
    console.log("Updating diagram!");        
    return this.httpClient.post<Diagram>(this.endpoint + 'save', diagram);
  }

  delete(id: string) : any {
    var diagram : Diagram = {
      diagramId: id
    };
    return this.httpClient.post<Diagram>(this.endpoint + 'delete', diagram);
  }

  create(diagram: Diagram) {
    return this.httpClient.post<Diagram>(this.endpoint + 'save', diagram).pipe(
      catchError(this.handleError)
    );  
  }

  updateData(data: UpdateData) {
    return this.httpClient.post<UpdateData>(this.endpoint + 'update-data', data).pipe(
      catchError(this.handleError)
    );  
  }
}