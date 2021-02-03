import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { CommandAction } from '../../../shared/hmi.constants'
import { DiagramData } from '../../../shared/models/userobject.model';

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
    @Inject(MAT_DIALOG_DATA) public data: any
  ) { }

  ngOnInit() {                    
    this.diagramId = this.data.diagramId;   
    this.diagramData = this.data.diagramData;
    this.name = this.diagramData.name,
    this.mRID = this.diagramData.mRID;
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
          this.has3PhaseLowerMapped = true;
          this.phase3LowerPath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpR.phs3.ctlVal')) {
          this.has3PhaseRaiseMapped = true;
          this.phase3RaisePath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpL.phsA.ctlVal')) {
          this.hasPhaseALowerMapped = true;
          this.phaseALowerPath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpR.phsA.ctlVal')) {
          this.hasPhaseARaiseMapped = true;
          this.phaseARaisePath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpL.phsB.ctlVal')) {
          this.hasPhaseBLowerMapped = true;
          this.phaseBLowerPath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpR.phsB.ctlVal')) {
          this.hasPhaseBRaiseMapped = true;
          this.phaseBRaisePath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpL.phsC.ctlVal')) {
          this.hasPhaseCLowerMapped = true;
          this.phaseCLowerPath = controlData.path;
        }
        else if (controlData.path.endsWith('.TapOpR.phsC.ctlVal')) {
          this.hasPhaseCRaiseMapped = true;
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
    window.open('/inspector?mrid=' + this.mRID, '_blank', 'toolbar=0,width=850,height=700', true);
    this.dialogRef.close({
      proceed: false      
    });
  }
}
