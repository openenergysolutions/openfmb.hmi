// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit } from '@angular/core';
import {CdkDragDrop, moveItemInArray, transferArrayItem} from '@angular/cdk/drag-drop';
import {FormControl} from '@angular/forms';
import { DiagramsService } from '../shared/services/diagrams.service';
import { Diagram } from '../shared/models/diagram.model';
import { Subscription } from 'rxjs';
import * as converter from 'xml-js';
import { getProfilesByModule, getModules } from '../shared/models/openfmb.model';
import { getCommands, getCommandsByType } from '../shared/models/commands.model';
import { MatSnackBar } from '@angular/material/snack-bar';
import { ActivatedRoute } from '@angular/router'
import { Hmi } from '../shared/hmi.constants'

@Component({
  selector: 'app-data-connect',
  templateUrl: './data-connect.component.html',
  styleUrls: ['./data-connect.component.scss']
})
export class DataConnectComponent implements OnInit {
  selectDiagramControl = new FormControl();
  diagrams: Diagram[] = [];
  graphItems: any[] = [];
  modules: any[] = [];
  profiles: any[] = [];
  dataPoints: any[] = [];
  graphModelData: any;
  commands: any[] = [];
  comparisons: string[] = ['equals', 'not-equals'];

  // Filter
  filter: "";
  temp = [];

  getItemSub: Subscription;
  selectedDiagramId: string;
  requestDiagramId: string;
  requestCellId: string;
  selectedGraphItemId: string; 
  selectedGraphItemMRID: string; 
  selectedGraphItem: any;
  selectedModule: string;
  selectedProfile: string;

  diagram: Diagram;
  graphModel : any;

  availablePoints = [];   
  currentPoints = [];
  currentControlPoints = [];
  currentVisibilityPoints = [];

  availableCommands = [];
  selectedCommand: string;

  graphItemControllable: boolean = false;
  graphItemDataConnectable: boolean = false;
  graphItemDataVisibility: boolean = false;

  constructor(
    private service: DiagramsService,
    private snack: MatSnackBar,
    private activateRoute : ActivatedRoute) {
      this.activateRoute.queryParams.subscribe(params => {
        this.requestDiagramId = params['id'];              
        this.requestCellId = params['cell'];
      });
    }

  ngOnInit(): void {
    this.getDiagrams();
    this.modules = getModules();
    this.commands = getCommands();
  }

  clamp(value, max) {
    return Math.max(0, Math.min(max, value));
  }

  transferArrayItem(currentArray, targetArray, currentIndex, targetIndex) {
    const from = this.clamp(currentIndex, currentArray.length - 1);
    const to = this.clamp(targetIndex, targetArray.length);
    if (currentArray.length) {
        // override the default behavior: not remove item from current array        
        targetArray.splice(to, 0, currentArray[from]);
    }
  }

  drop(event: CdkDragDrop<string[]>) {    
    if (event.previousContainer === event.container) {
      moveItemInArray(event.container.data, event.previousIndex, event.currentIndex);
    } else {
      if (this.selectedDiagramId && this.selectedGraphItemId) {
        if (!event.container.data) {
          event.container.data = [];
        }

        if (this.selectedGraphItem.type !== 'measure-box' && event.container.data.length > 0) {
          this.snack.open('Only one mapping is allowed.', 'OK', { duration: 4000 });
         
        }
        else 
        {
          this.transferArrayItem(event.previousContainer.data,
                          event.container.data,
                          event.previousIndex,
                          event.currentIndex);
        }
      }
      else {
        this.snack.open('Please select diagram and graph item.', 'OK', { duration: 4000 })
      }
    }
  }

  getDiagrams() {
    this.getItemSub = this.service.getAll()
      .subscribe(data => {
        this.diagrams = data;
        if (this.requestDiagramId) {
          this.selectedDiagramId = this.requestDiagramId;
          this.onDiagramChanged(this.selectedDiagramId);
        }         
      }, error => {
        console.error(error);
        this.snack.open(error, 'OK', { duration: 4000 });
      });
  }

  onDiagramChanged(id: string) {
    console.log("data-connect:: on diagram changed: " + id);
    this.selectedDiagramId = id;    
    this.service.get(id).subscribe(data => {
      this.diagram = data;

      if (!this.diagram) {
        this.snack.open('Unable to retrieve digram with ID ' + id, 'OK', { duration: 4000 });
        return;
      }
  
      var xml = this.diagram.data;
      
      this.graphModel = converter.xml2js(xml);        
  
      var elem = this.graphModel.elements[0].elements[0];
      
      this.graphItems = [];
      this.selectedGraphItemId = null; 
      this.selectedGraphItemMRID = null;     
      this.currentPoints = [];
      this.currentControlPoints = [];
      this.currentVisibilityPoints = [];
  
      for(var i = 0; i < elem.elements.length; ++i) {
        var e = elem.elements[i];
        if (e.name === "mxCell") {        
          if (e.attributes) {          
            if (e.attributes.vertex === "1") { // Search vor vertex: "1"            
              var child = e.elements[0];
              for(var k = 0; k < child.elements.length; ++k) {
                var objElement = child.elements[k];
                if (objElement.name === "Object") {
                  if (objElement.attributes) {
                    if (objElement.attributes.as === "userObject") { // found "userObject"
                      // User object
                      if (objElement.elements)
                      {
                        if (!Hmi.isDataConnectable(objElement.attributes.type)) {
                          continue;
                        }
                        
                        var displayData = null;
                        var controlData = null;
                        var visibilityData = null;

                        for(var j = 0; j < objElement.elements.length; ++j) {
                          if (objElement.elements[j].attributes.as == "displayData")
                          {
                            displayData = objElement.elements[j];
                            if (!displayData.elements) {
                              displayData.elements = [];
                            }  
                          }
                          else if (objElement.elements[j].attributes.as == "controlData")
                          {
                            controlData = objElement.elements[j];
                            if (!controlData.elements) {
                              controlData.elements = [];
                            }  
                          }
                          else if (objElement.elements[j].attributes.as == "visibilityData")
                          {
                            visibilityData = objElement.elements[j];
                            if (!visibilityData.elements) {
                              visibilityData.elements = [];
                            }  
                          }
                        } 
                        
                        if (visibilityData == null) {  // backward compability
                          visibilityData = {
                            elements: []
                          };
                        }
                        
                        var vertex = {
                          id: e.attributes.id,
                          name: objElement.attributes.name || "name",
                          label: objElement.attributes.label || "label",                          
                          mRID: objElement.attributes.mRID,                        
                          displayData: displayData,  
                          controlData: controlData,
                          visibilityData: visibilityData,                         
                          type: objElement.attributes.type
                        };
  
                        if (e.attributes.id === this.requestCellId) {
                          this.selectedGraphItemId = this.requestCellId;                          
                          this.selectedGraphItem = vertex;
                          this.selectedGraphItemMRID = vertex.mRID;
                        }
                        this.graphItems.push(vertex);                           
                      }                    
                    }
                  }
                }
              }            
            }
          }        
        }
      } 
      
      if (this.requestCellId) {
        
        this.selectedGraphItemId = this.requestCellId;
        this.requestCellId = null;
        this.onGraphItemChanged(this.selectedGraphItemId);
      }
    });      
  }

  onGraphItemChanged(id: string) {    
    this.currentPoints = [];
    this.currentControlPoints = [];
    this.currentVisibilityPoints = [];
    for(var i = 0; i < this.graphItems.length; ++i) {
      if (this.graphItems[i].id === id) {        
        this.selectedGraphItem = this.graphItems[i];
        this.selectedGraphItemMRID = this.selectedGraphItem.mRID;
        this.currentPoints = this.graphItems[i].displayData.elements;
        this.currentControlPoints = this.graphItems[i].controlData.elements; 
        this.currentVisibilityPoints = this.graphItems[i].visibilityData.elements;         
        this.graphItemControllable = Hmi.isControllable(this.graphItems[i].type);
        this.graphItemDataConnectable = Hmi.isDataConnectable(this.graphItems[i].type);
        this.graphItemDataVisibility = Hmi.isVisibilitySupport(this.graphItems[i].type);
        break;
      }
    }
  }  

  removePoint(item) {
    console.log("remove data:: " + item);

    if (item) {
      for(let index = this.currentPoints.length - 1; index >=0; --index) {
        var obj = this.currentPoints[index];
        if (obj === item) {        
          this.currentPoints.splice(index, 1);
          break;
        }
      }
    }
    else {
      console.error("Unable to delete connected data point.  item=" + item);
    }
  }

  removeControlPoint(item) {
    console.log("remove data:: " + item);

    if (item) {
      for(let index = this.currentControlPoints.length - 1; index >=0; --index) {
        var obj = this.currentControlPoints[index];
        if (obj === item) {        
          this.currentControlPoints.splice(index, 1);
        }
      }
    }
    else {
      console.error("Unable to delete connected control data point.  item=" + item);
    }
  }

  removeVisibilityPoint(item) {
    console.log("remove data:: " + item);

    if (item) {
      for(let index = this.currentVisibilityPoints.length - 1; index >=0; --index) {
        var obj = this.currentVisibilityPoints[index];
        if (obj === item) {        
          this.currentVisibilityPoints.splice(index, 1);
          break;
        }
      }
    }
    else {
      console.error("Unable to delete connected data point.  item=" + item);
    }
  }

  addPoint(item: any) {
    console.log("add data:: " + item);
    if (this.selectedGraphItem.type !== 'measure-box' && this.currentPoints.length > 0) {
      this.snack.open('Only one mapping is allowed.', 'OK', { duration: 4000 });
    }
    else {
      this.currentPoints.push({ ...item});
    }
  }

  addControlPoint(item: any) {
    console.log("add control data:: " + item);    
    if (!Hmi.isVoltageRegulator(this.selectedGraphItem.type) && this.currentControlPoints.length > 0) {
      this.snack.open('Only one mapping is allowed.', 'OK', { duration: 4000 });
    }
    else {
      this.currentControlPoints.push(item);
    }
  }

  addVisibilityPoint(item: any) {
    console.log("add visibility data:: " + item);
    this.currentVisibilityPoints.push({ ...item});
  }

  onModuleChanged(name: string) {    
    this.availablePoints = this.temp = [];
    this.filter = "";
    if (name) {
      this.profiles = getProfilesByModule(name);
    }
  }
  
  onCommandTypeChanged(name: string) {
    this.availableCommands = [];
    if (name) {
      this.availableCommands = getCommandsByType(name);
    }
  }

  onProfileChanged(name: string) {
    this.filter = "";
    this.availablePoints = this.temp = [];
    if (name) {
      const currentProfile = this.profiles.find((elem) => elem.name === name);
      if (currentProfile) {
        this.availablePoints = this.temp = this.setSelectedFields(JSON.parse(JSON.stringify(currentProfile.topics)), this.availablePoints);
      }
    }
  }

  private setSelectedFields(allFields: any[], selectedFields: any[]): any[] {
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

  updateFilter(event) {
    const val = event.target.value.toLowerCase();
    
    const rows = this.temp.filter(function(d) {
      if (d.attributes) {
        if (d.attributes.label && d.attributes.label.toString().toLowerCase().indexOf(val) > -1) {
          return true;
        }
        else if (d.attributes.name && d.attributes.name.toString().toLowerCase().indexOf(val) > -1) {
          return true;
        }
        else if (d.attributes.path && d.attributes.path.toString().toLowerCase().indexOf(val) > -1) {
          return true;
        }
      }
    });

    this.availablePoints = rows;
  }

  updateDiagram() {
    try {
      if (this.selectedDiagramId && this.selectedGraphItemId) {      
        for(var i = 0; i < this.graphItems.length; ++i) {
          var item = this.graphItems[i];
          if (item.id === this.selectedGraphItemId) {                             
            var xml = converter.js2xml(this.graphModel);
            console.log(xml);
            this.diagram.data = xml;                        
            this.service.update(this.diagram).subscribe(
              response => {
                //console.log("Updated diagram:: " + response),
                this.snack.open('Diagram is updated.', 'OK', { duration: 2000 });        
              },
              err => console.log(err)
            );            
            break;
          }
        }
      }
      else {
        console.error("Program error:: update diagram settings without selecting diagramId and vertex id.");
      }
    }
    catch (e) {
      console.error(e);
      this.snack.open('Error saving diagram.  Check logs for more information.', 'OK', { duration: 4000 });
    }
  }

  truncateString(fullStr: string) : string {
    const strLen = 100;

    if (fullStr.length <= strLen) {
      return fullStr;
    }
    
    let separator = '...';
    
    var sepLen = separator.length,
        charsToShow = strLen - sepLen,
        frontChars = Math.ceil(charsToShow/2),
        backChars = Math.floor(charsToShow/2);
    
    return fullStr.substr(0, frontChars) + 
           separator + 
           fullStr.substr(fullStr.length - backChars);
  }
}
