// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { CommandAction } from '../../../shared/hmi.constants'
import { DiagramData } from '../../../shared/models/userobject.model';
import { Authorization } from '../../../shared/models/user.model';
import { JwtAuthService } from "../../../shared/services/auth/jwt-auth.service";

@Component({
  selector: 'app-regulator-dialog',
  templateUrl: './regulator-dialog.component.html',
  styleUrls: ['./regulator-dialog.component.scss']
})
export class RegulatorDialogComponent implements OnInit {  
  status: string;
  name: string;    
  diagramId: string;
  mRID: string;
  description: string;
  showDescription: boolean = false;
  diagramData: DiagramData;  
  hasDataMapped: boolean = false;
  has3PhaseLowerMapped: boolean = false;
  has3PhaseRaiseMapped: boolean = false;
  hasPhaseALowerMapped: boolean = false;
  hasPhaseARaiseMapped: boolean = false;
  hasPhaseBLowerMapped: boolean = false;
  hasPhaseBRaiseMapped: boolean = false;
  hasPhaseCLowerMapped: boolean = false;
  hasPhaseCRaiseMapped: boolean = false;

  phase3LowerPath: string = "";
  phase3RaisePath: string = "";
  phaseALowerPath: string = "";
  phaseARaisePath: string = "";
  phaseBLowerPath: string = "";
  phaseBRaisePath: string = "";
  phaseCLowerPath: string = "";
  phaseCRaisePath: string = "";

  lastUpdate: string;
  hasLastUpdate: boolean = false;    

  constructor(
    public dialogRef: MatDialogRef<RegulatorDialogComponent>,
    private jwtService: JwtAuthService,    
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {  
    const canControl = Authorization.canControl(this.jwtService.getUserRole());                  
    this.diagramId = this.data.diagramId;   
    this.diagramData = this.data.diagramData;
    this.name = this.diagramData.name,
    this.mRID = this.diagramData.mRID;
    this.description = this.diagramData.description;
    this.showDescription = this.description && this.description !== "";
    this.status = this.diagramData.tag;
    this.lastUpdate = this.diagramData.lastUpdate;
    if (this.lastUpdate) {
      this.hasLastUpdate = true;
    }
    
    if (!this.diagramData.controlData || this.diagramData.controlData.length == 0) {      
      this.hasDataMapped = false;
    }
    else {
      this.hasDataMapped = true;

      for(var i = 0; i < this.diagramData.controlData.length; ++i) {
        var controlData = this.diagramData.controlData[i];

        if (controlData.path.endsWith('.TapOpL.phs3.ctlVal')) {
          this.has3PhaseLowerMapped = canControl;
          this.phase3LowerPath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpR.phs3.ctlVal')) {
          this.has3PhaseRaiseMapped = canControl;
          this.phase3RaisePath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpL.phsA.ctlVal')) {
          this.hasPhaseALowerMapped = canControl;
          this.phaseALowerPath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpR.phsA.ctlVal')) {
          this.hasPhaseARaiseMapped = canControl;
          this.phaseARaisePath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpL.phsB.ctlVal')) {
          this.hasPhaseBLowerMapped = canControl;
          this.phaseBLowerPath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpR.phsB.ctlVal')) {
          this.hasPhaseBRaiseMapped = canControl;
          this.phaseBRaisePath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpL.phsC.ctlVal')) {
          this.hasPhaseCLowerMapped = canControl;
          this.phaseCLowerPath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpR.phsC.ctlVal')) {
          this.hasPhaseCRaiseMapped = canControl;
          this.phaseCRaisePath = controlData.path;
        }
      }
    }        
  }
  
  onClose(): void {    
    this.dialogRef.close();
  }  

  onLowerPhs3() : void {
    if (this.has3PhaseLowerMapped) {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.TAP_LOWER_PHS3,
        path: this.phase3LowerPath
      });
    }
    else {
      alert('3-Phase Tap Lower is not mapped.');
    }
  }
  onRaisePhs3() : void {
    if (this.has3PhaseRaiseMapped) {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.TAP_RAISE_PHS3,
        path: this.phase3RaisePath
      });
    }
    else {
      alert('3-Phase Tap Raise is not mapped.');
    }
  }
  onLowerPhsA() : void {
    if (this.hasPhaseALowerMapped) {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.TAP_LOWER_PHSA,
        path: this.phaseALowerPath
      });
    }
    else {
      alert('Phase A Tap Lower is not mapped.');
    }
  }
  onRaisePhsA() : void {
    if (this.hasPhaseARaiseMapped) {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.TAP_RAISE_PHSA,
        path: this.phaseARaisePath
      });
    }
    else {
      alert('Phase A Tap Raise is not mapped.');
    }
  }
  onLowerPhsB() : void {
    if (this.hasPhaseBLowerMapped) {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.TAP_LOWER_PHSB,
        path: this.phaseBLowerPath
      });
    }
    else {
      alert('Phase B Tap Lower is not mapped.');
    }
  }
  onRaisePhsB() : void {
    if (this.hasPhaseBRaiseMapped) {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.TAP_RAISE_PHSB,
        path: this.phaseBRaisePath
      });
    }
    else {
      alert('Phase B Tap Raise is not mapped.');
    }
  }
  onLowerPhsC() : void {
    if (this.hasPhaseCLowerMapped) {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.TAP_LOWER_PHSC,
        path: this.phaseCLowerPath
      });
    }
    else {
      alert('Phase C Tap Lower is not mapped.');
    }
  }
  onRaisePhsC() : void {
    if (this.hasPhaseCRaiseMapped) {
      this.dialogRef.close({
        proceed: true,
        action: CommandAction.TAP_RAISE_PHSC,
        path: this.phaseCRaisePath
      });
    }
    else {
      alert('Phase C Tap Raise is not mapped.');
    }
  }

  onMessageInspector(): void {
    window.open('/inspector?mrid=' + this.mRID, '_blank', 'toolbar=0,width=850,height=700');
    this.dialogRef.close({
      proceed: false      
    });
  }
}
