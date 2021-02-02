import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router'
import { v4 as uuidv4 } from 'uuid';
import { WebSocketService } from '../core/services/web-socket.service';
import { JwtAuthService } from '../shared/services/auth/jwt-auth.service';
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-inspector-dialog',
  templateUrl: './inspector-dialog.component.html',
  styleUrls: ['./inspector-dialog.component.scss']
})
export class InspectorDialogComponent implements OnInit { 
  mrid: string;
  sessionId: string;

  constructor(
    private wsService: WebSocketService,
    private jwtAuth: JwtAuthService,
    private router : ActivatedRoute,
    private snack: MatSnackBar,
  ) { 
    // Check Auth Token is valid
    this.jwtAuth.checkTokenIsValid().subscribe();
    
    this.router.queryParams.subscribe(params => {
      this.mrid = params['mrid'];      
    });
  }  

  ngOnInit() {                 
  }  

  ngAfterViewInit() {       
    if (this.mrid) {  
      this.sessionId = uuidv4();    
      this.connect(this.sessionId);          
    }
  }

  connect(sessionId: string) {
    this.wsService.connect(sessionId);
    this.wsService.wsConnection$
      .subscribe(
        (msg) => {
          if (msg) {            
            this.register();
          }
          else {
            this.snack.open('Connection to server has lost.', 'OK', { duration: 4000 });                                   
          }
        },
        (error) => {
          console.log(error);
        }
      );
    this.wsService.wsMessages$ 
      .subscribe(
        (message) => {
          this.onReceivedMessage(message);
        },
        (error) => {
          console.log(error);
        }
      );
  }

  register() {            
    var request = {
      session_id: this.sessionId,
      topics: [
        {
          name: '*',
          mrid: this.mrid,
        }
      ]
    };    
    this.wsService.sendWsData(request);
  }

  onReceivedMessage(message: any)
  {
  }
}
