import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { DiagramsService } from '../../../shared/services/diagrams.service';
import { DiagramData, LinkData, StatusDefinition } from '../../../shared/models/userobject.model'
import { Diagram } from '../../../shared/models/diagram.model';
import { Symbol } from '../../../shared/hmi.constants'
import { Router } from '@angular/router';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-properties-dialog',
  templateUrl: './properties-dialog.component.html',
  styleUrls: ['./properties-dialog.component.scss']
})
export class PropertiesDialogComponent implements OnInit {
  filterData: any;
  label: string;
  name: string;
  displayDataLabel = "Reading/Status";
  controlDataLabel = "Control";
  diagramId: string;
  mRID: string;
  fontSize: number;
  containerWidth: number;
  foreColor: any;
  backgroundColor: any;
  changeStyleAllowed: boolean;
  changeWidthAllowed: boolean;
  changeBackgroundAllowed: boolean;
  linkAllowed: boolean;
  deviceTypeMapping: string;
  displayData: any[];
  controlData: any[];
  defaultFields = [];
  deviceTypeOptions = [];
  mRIdOptions: string[] = [];
  navigateToDataConnection: boolean = false;
  dataConnectAllowed: boolean;
  statusDefinitionAllowed: boolean = false;
  
  // For link
  selectedDiagramId: string;
  selectedLinkTarget: string;
  diagrams: Diagram[] = [];
  getItemSub: Subscription;
  linkTargetOptions: string[];

  // status definition
  statusDefinitions: StatusDefinition[];
  statusColors: string[] = ['gray', 'green', 'yellow', 'red'];

  constructor(
    public dialogRef: MatDialogRef<PropertiesDialogComponent>,
    private router: Router,
    private service: DiagramsService,
    @Inject(MAT_DIALOG_DATA) public data: DiagramData
  ) { }

  ngOnInit() {    
    this.filterData = this.data;
    this.label = this.data.label;
    this.name = this.data.name,
    this.mRID = this.data.mRID;
    this.fontSize = this.data.fontSize;
    this.containerWidth = this.data.containerWidth;
    this.foreColor = this.data.foreColor;
    this.changeStyleAllowed = this.data.changeStyleAllowed;
    this.backgroundColor = this.data.backgroundColor;
    this.diagramId = this.data.diagramId;
    this.deviceTypeMapping = this.data.deviceTypeMapping; 
    this.changeWidthAllowed = this.data.type === Symbol.measureBox;
    this.changeBackgroundAllowed = this.data.type === Symbol.measureBox;
    this.linkAllowed = this.data.type === Symbol.button;
    this.dataConnectAllowed = this.data.type !== Symbol.label;
    this.statusDefinitionAllowed = this.data.type === Symbol.twoStateButton;

    if (this.linkAllowed) {      
      this.linkTargetOptions = ['_blank', '_top',];

      if (this.data.linkData) {
        this.selectedDiagramId = this.data.linkData.diagramId;
        this.selectedLinkTarget = this.data.linkData.target;
      }

      this.getDiagrams();
    }

    if (this.statusDefinitionAllowed) {
      if (this.data.statusDefinition) {
        this.statusDefinitions = this.data.statusDefinition;
      }
      else {
        this.statusDefinitions = [];
      }
    }
    
    this.dialogRef.updatePosition({
      top: `${this.filterData.top}px`,
      left: `${this.filterData.left}px`
    });
    if (this.filterData.displayData) {
      this.displayData = [...this.filterData.displayData];
    } 
    if (this.filterData.controlData) {
      this.controlData = [...this.filterData.controlData];
    } 
  }

  getDiagrams() {
    this.getItemSub = this.service.getAll()
      .subscribe(data => {
        this.diagrams = data;        
      })
  }

  // save all grid item data
  onSave(): void {
    console.log(this.backgroundColor);

    var foreC = null;
    if (this.foreColor) {
      if (this.foreColor.hex) {
        foreC = '#' + this.foreColor.hex;
      }
      else {
        var temp = '' + this.foreColor;
        if (temp.startsWith('#')) {
          foreC = temp;
        }
      }
    }

    var backgroundC = null;
    if (this.backgroundColor) {
      if (this.backgroundColor.hex) {
        backgroundC = '#' + this.backgroundColor.hex;
      }
      else {
        var temp = '' + this.backgroundColor;
        if (temp.startsWith('#')) {
          backgroundC = temp;
        }
      }
    } 

    var linkData: LinkData = null;
    
    if (this.linkAllowed) {
      linkData = {
        diagramId: this.selectedDiagramId,
        target: this.selectedLinkTarget
      };
    }

    this.dialogRef.close({
      label: this.label,
      name: this.name,
      displayData: this.displayData,
      controlData: this.controlData,
      mRID: this.mRID,
      fontSize: this.fontSize,
      containerWidth: this.containerWidth,
      foreColor: foreC,
      backgroundColor: backgroundC,
      linkData: linkData,
      deviceTypeMapping: this.deviceTypeMapping,
      navigateToDataConnection: this.navigateToDataConnection,
      statusDefinition: this.statusDefinitions
    });
  }

  // close modal window
  onNoClick(): void {
    this.dialogRef.close();
  }  

  setSelectedFields(allFields: any[], selectedFields: any[]): any[] {
    if (selectedFields.length) {
     return allFields.map( elem => {
       selectedFields.forEach(selectElem => {
         if (selectElem.value === elem.value) {
           elem.selected = true;
         }
       });
        return elem;
      });
    }
    return allFields;
  }    
  
  // navigate to data connect screen
  dataConnect() { 
    this.navigateToDataConnection = true;   
    this.onSave();    
  }

  addStatusDefinition() {
    const def: StatusDefinition = {
      value: 0,      
      color: "gray"
    };
    this.statusDefinitions.push(def);
  }

  removeStatusDefinition(item: StatusDefinition) {
    if (item) {
      for(let index = this.statusDefinitions.length - 1; index >=0; --index) {
        var obj = this.statusDefinitions[index];
        if (obj === item) {        
          this.statusDefinitions.splice(index, 1);
        }
      }
    }
    else {
      console.error("Unable to delete status definition.  item=" + item);
    }
  }
}
