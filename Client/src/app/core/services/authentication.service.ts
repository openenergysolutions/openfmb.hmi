// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
// TODO: Cleanup comments
// import { HttpClient } from '@angular/common/http';
// import { Auth } from '../models/auth';
// import { environment } from 'src/environments/environment';
// import { map } from 'rxjs/operators';
import { AuthService, User } from '@auth0/auth0-angular';
import { AuthConstant } from '../constants/auth-constant';

// TODO: (Maybe) Consolidate Auth0 authentication here
@Injectable({
  providedIn: 'root'
})
export class AuthenticationService {
  // private currentUserSubject: BehaviorSubject<Auth>;
  private currentUser: User;
  public currentUser$: Observable<User>;

  constructor(private auth: AuthService) {
    this.currentUser$ = this.auth.user$;
    this.currentUser$.subscribe(user => {
      this.currentUser = user;
    });
    // this.currentUserSubject = new BehaviorSubject<Auth>(JSON.parse(localStorage.getItem('currentUser')));
    // this.currentUser = this.currentUserSubject.asObservable();
  }

  public get currentUserValue(): User {
    return this.currentUser;
  }

  public get currentUserRoles(): string[] {
    return this.currentUser.user[AuthConstant.ROLES]
  }

  // validate() {
  //   return this.http.get<any>(`${environment.apiUrl}/users/validate`)
  //     .pipe(map(() => {
  //       return true;
  //     }));
  // }

  // login(username: string, password: string) {
  //   return this.http.post<any>(`${environment.apiUrl}/users/login`, { username, password })
  //     .pipe(map(auth => {
  //       // store user details and jwt token in local storage to keep user logged in between page refreshes
  //       localStorage.setItem('currentUser', JSON.stringify(auth));
  //       this.currentUserSubject.next(auth);
  //       return auth;
  //     }));
  // }

  // logout() {
  //   // remove user from local storage to log user out
  //   localStorage.removeItem('currentUser');
  //   this.currentUserSubject.next(null);
  // }
}
