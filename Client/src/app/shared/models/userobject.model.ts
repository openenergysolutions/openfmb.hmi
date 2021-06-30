// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

export interface DiagramData {
    diagramId?: string,
    label?: string,
    name?: string,
    mRID?: string,
    deviceTypeMapping?: string, // used anywhere?
    left?: number,
    top?: number,
    type?: string, // type of graph item
    func?: string, // button function 
    verb?: string, // command verb
    linkData?: LinkData, // store link data when func == ButtonFunction.link
    textAlign?: string,
    fontSize?: number,
    fontStyle?:string,
    foreColor?: string,
    backgroundColor?: string,
    containerWidth?: number,
    containerHeight?: number,
    changeStyleAllowed?: boolean,    
    tag?: any,
    lastUpdate?: any,
    statusDefinition?: StatusDefinition[],
    arrowDirection?: ArrowDirection,
    displayData: any[],
    controlData: any[]    
};

export interface LinkData {
    url?: string,
    diagramId?: string,
    target?: string
}

export interface StatusDefinition {
    value?: any,    
    color?: string
}

export interface ArrowDirection {
    positive?: string,
    negative?: string,
    neutral?: string,
    positiveColor?: string,
    negativeColor?: string,
    neutralColor?: string
}