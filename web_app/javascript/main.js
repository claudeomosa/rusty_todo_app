const renderItems = (items, processType, elemID,  processFunction) => {
    let itemsMeta = []
    let placeholder = "<div>"
    for (let i = 0; i < items.length; i++) {
        const element = items[i];
        let name = element["name"];
        let placeholderId = processType + "-" + name.replaceAll(" ", "-");
        placeholder += "<div  class='itemContainer'>" +'<p>'+ name + '</p>' +  '<div class="actionButton" ' + 'id="' + placeholderId + '">'+ processType +'</div>' + "</div>";
        itemsMeta.push({"id": placeholderId, "name": name});
    }
    placeholder+="</div>"
    document.getElementById(elemID).innerHTML = placeholder;
    for (let i = 0; i < itemsMeta.length; i++) {
        document.getElementById(
            itemsMeta[i]["id"]).addEventListener(
            "click", processFunction);
    }
}

const apiCall = (url, method) => {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener("readystatechange", function () {
        if (this.readyState === this.DONE) {
            renderItems(JSON.parse(this.responseText)["pending_items"], "edit", "pendingItems", editItem);
            renderItems(JSON.parse(this.responseText)["done_items"], "delete", "doneItems", deleteItem);
        }
    })
    xhr.open(method, url);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.setRequestHeader("user-token", "token");
    return xhr;
}

const editItem = () => {
    let name = this.id.replaceAll("-", " ").replace("edit ", "");
    let call = apiCall("/app/items/edit", "POST");
    let json = {
        "name": name,
        "status": "DONE"
    };
    call.send(JSON.stringify(json));
}

const deleteItem = () => {
    let name = this.id.replaceAll("-", " ").replace("delete ", "");
    let call = apiCall("/app/items/delete", "POST");
    let json = {
        "name": name,
        "status": "DONE"
    };
    call.send(JSON.stringify(json));
}

const getItems = () => {
    let call = apiCall("/app/items", "GET");
    call.send();
}

getItems();

const addItem = () => {
    let name = document.getElementById("name");
    let call = apiCall("/app/items/create" + name.value, "POST");
    call.send();
}