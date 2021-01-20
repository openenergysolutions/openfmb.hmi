var hmiPopupWindows = [];

function navigateToDiagram(diagramId, target) {

    if (hmiPopupWindows != null)
    {          
        for(var i = 0; i < hmiPopupWindows.length; ++i) {
            try {
                var w = hmiPopupWindows[i];
                if (w.location.href.endsWith(diagramId)) {   
                    console.log(w);
                    if (!w.closed) {                                        
                        w.focus();
                        return;
                    }
                    else {
                        hmiPopupWindows.splice(i, 1);                    
                    }
                }
            } catch (e) {}
        }                
    }

    var w = window.open('/hmi?id=' + diagramId, target, 'toolbar=0,width=750,height=700', true);
    hmiPopupWindows.push(w);    
}