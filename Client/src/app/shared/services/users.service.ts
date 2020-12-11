import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http'
import { config } from '../../../config'
import { User } from '../models/user.model'
import { Observable, of } from 'rxjs';

@Injectable({
  providedIn: 'root'
})

export class UserService {
    
  constructor(private httpClient: HttpClient) { }

  userList = [
    {
        "userId": "f033d088-8ace-48a6-8cec-31befd43308f",
        "userName": "admin",
        "firstName": "User",
        "lastName": "Admin",
        "role": "admin"
    },
    {
        "userId": "dd214b2a-484d-42bf-8ac5-8df38de1f3cb",
        "userName": "user1",
        "firstName": "User",
        "lastName": "Engineer",
        "role": "engineer"
    },
    {
        "userId": "ed0ca7ce-a516-4c2e-afc9-dacda9cde6b9",
        "userName": "user2",
        "firstName": "User",
        "lastName": "Viewer",
        "role": "viewer"
    }
  ];

  getAll() : Observable<any> {    
    return of(this.userList);
  }

  get(id: string) : any {
    for(var i = 0; i < this.userList.length; ++i) {
      if (this.userList[i].userId === id) {
        return this.userList[i];
      }
    }
    return null;
  }

  update(user: User) : Observable<any> {
    console.log("Update user:: " + user);

    return this.getAll();
  }

  delete(id: string) {
    console.log("TODO:: implement delete user in user service.");
    return this.getAll();
  }

  create(user: User) {
    console.log("TODO:: implement create user in user service.");
    return this.getAll();
  }
}