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