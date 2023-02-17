import { HttpHandler, HttpInterceptor, HttpRequest } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { AuthService } from '@auth0/auth0-angular';
import { switchMap} from 'rxjs/operators';

@Injectable()
export class CustomAuthInterceptor implements HttpInterceptor {

  constructor(private auth: AuthService) {}

  intercept(req: HttpRequest<any>, next: HttpHandler) {
    return this.auth.getAccessTokenSilently({audience: "openfmb-hmi"}).pipe(
      switchMap(access_token=>{
        const authReq = req.clone({
          headers: req.headers.set('Authorization', `Bearer ${access_token}`),
        });
        return next.handle(authReq);
      }));
  }
}