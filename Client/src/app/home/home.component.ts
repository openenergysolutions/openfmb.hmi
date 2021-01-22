import { Component, OnInit } from '@angular/core';
import { JwtAuthService } from "../shared/services/auth/jwt-auth.service";
import { Authorization } from '../shared/models/user.model';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {
  canEditDiagram: boolean = false;
  constructor(private jwtService: JwtAuthService) { }

  ngOnInit(): void {
    this.canEditDiagram = Authorization.canEditDiagram( this.jwtService.getUserRole());
  }

}
