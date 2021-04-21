import { Injectable } from '@angular/core';
import { HttpClient, HttpErrorResponse } from '@angular/common/http'
import { environment } from '../../../environments/environment';
import { User } from '../models/user.model'
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/internal/operators';

@Injectable({
  providedIn: 'root'
})

export class UserService {
  private endpoint = environment.apiUrl;  
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

  getAll() : Observable<any> {        
    return this.httpClient.get<User>(this.endpoint + 'get-users').pipe(
      catchError(this.handleError)
    );
  }

  update(user: User) : Observable<any> {    
    return this.httpClient.post<User>(this.endpoint + 'update-user', user).pipe(
      catchError(this.handleError)
    );  
  }

  delete(id: string) : Observable<any>  {
    var user : User = {
      id: id,
      username: "",
      displayname: "",
      pwd: "",
      role: ""
    };
    return this.httpClient.post<User>(this.endpoint + 'delete-user', user);
  }

  create(user: User) : Observable<any> {
    return this.httpClient.post<User>(this.endpoint + 'create-user', user).pipe(
      catchError(this.handleError)
    );      
  }
}