export const getCommands = () => {
    return Object.keys(COMMANDS);
};
  
export const getCommandsByType = (type: string) => {
    return COMMANDS[type] ? COMMANDS[type] : []
};

export const COMMANDS = {
    "MicrogridControl": [        
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "ResetDevices",
                "path": "ResetDevices",
                "type": "",
				"measurement": ""               
            }
        }, 
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "InitiateIsland",
                "path": "InitiateIsland",
                "type": "",
				"measurement": ""               
            }
        },        
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "InitiateGridConnect",
                "path": "InitiateGridConnect",
                "type": "",
				"measurement": ""               
            }
        }, 
        {   
            "type": "element",
			"name": "Object",          
            "attributes": {
                "label": "",
                "name": "EnableNetZero",
                "path": "EnableNetZero",
                "type": "",
				"measurement": ""               
            }
        }, 
        {   
            "type": "element",
			"name": "Object",          
            "attributes": {
                "label": "",
                "name": "DisableNetZero",
                "path": "DisableNetZero",
                "type": "",
				"measurement": ""               
            }
        },            
        {   
            "type": "element",
			"name": "Object",          
            "attributes": {
                "label": "",
                "name": "ReconnectPretestOne",
                "path": "ReconnectPretestOne",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "ReconnectPretestTwo",
                "path": "ReconnectPretestTwo",
                "type": "",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "ReconnectTest",
                "path": "ReconnectTest",
                "type": "",
				"measurement": ""               
            }
        },
        {  
            "type": "element",
			"name": "Object",          
            "attributes": {
                "label": "",
                "name": "SubscribeDevice",
                "path": "SubscribeDevice",
                "type": "",
				"measurement": ""              
            }
        }       
    ],
    "DeviceControl": [
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "EnableSolarInverter",
                "path": "EnableSolarInverter",
                "type": "",
				"measurement": ""               
            }
        },
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "DisableSolarInverter",
                "path": "DisableSolarInverter",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EnableLoadbank",
                "path": "EnableLoadbank",
                "type": "",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "DisableLoadbank",
                "path": "DisableLoadbank",
                "type": "",
				"measurement": ""               
            }
        },
        {   
            "type": "element",
			"name": "Object",         
            "attributes": {
                "label": "",
                "name": "EssStart",
                "path": "EssStart",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EssDischarge",
                "path": "EssDischarge",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EssSocManage",
                "path": "EssSocManage",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EssSocLimits",
                "path": "EssSocLimits",
                "type": "",
				"measurement": ""               
            }
        },         
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "EssStop",
                "path": "EssStop",
                "type": "",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "GeneratorOn",
                "path": "GeneratorOn",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "GeneratorDisabled",
                "path": "GeneratorDisabled",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "GeneratorEnabled",
                "path": "GeneratorEnabled",
                "type": "",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "GeneratorOff",
                "path": "GeneratorOff",
                "type": "",
				"measurement": ""               
            }
        },
        {      
            "type": "element",
			"name": "Object",      
            "attributes": {
                "label": "",
                "name": "SwitchOneOpen",
                "path": "SwitchOneOpen",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "SwitchOneClosed",
                "path": "SwitchOneClosed",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "ToggleSwitchOne",
                "path": "ToggleSwitchOne",
                "type": "",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "SwitchTwoOpen",
                "path": "SwitchTwoOpen",
                "type": "",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "SwitchTwoClosed",
                "path": "SwitchTwoClosed",
                "type": "",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "ToggleSwitchTwo",
                "path": "ToggleSwitchTwo",
                "type": "",
				"measurement": ""               
            }
        },
        {      
            "type": "element",
			"name": "Object",      
            "attributes": {
                "label": "",
                "name": "BreakerThreeOpen",
                "path": "BreakerThreeOpen",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "BreakerThreeClosed",
                "path": "BreakerThreeClosed",
                "type": "",
				"measurement": ""               
            }
        },
        {     
            "type": "element",
			"name": "Object",       
            "attributes": {
                "label": "",
                "name": "ToggleBreakerThree",
                "path": "ToggleBreakerThree",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "SwitchFourOpen",
                "path": "SwitchFourOpen",
                "type": "",
				"measurement": ""               
            }
        },
        { 
            "type": "element",
			"name": "Object",           
            "attributes": {
                "label": "",
                "name": "SwitchFourClosed",
                "path": "SwitchFourClosed",
                "type": "",
				"measurement": ""               
            }
        },
        {    
            "type": "element",
			"name": "Object",        
            "attributes": {
                "label": "",
                "name": "ToggleSwitchFour",
                "path": "ToggleSwitchFour",
                "type": "",
				"measurement": ""               
            }
        }
    ]
};