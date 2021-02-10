
//----connection------
// var isReceive = false;
function getAccount(accountId) {
    // console.log('this.connection',window.near.connection)
    return new nearApi.Account(window.near.connection, accountId)
}

//-------new Contract-------
async function createNewContract(fundingKey){
    layer.open({
        type: 2
    });

    const account = getAccount(nearConfig.contractName);
    const userName=wallet.getAccountId();

    // let packet_keys = getQueryVariable('packet_keys').split('-');
    let packet_keys;
    if(getQueryVariable('packet_keys')){
        packet_keys = decodeURIComponent(getQueryVariable('packet_keys')).split('-');
    }else {
        packet_keys = decodeURIComponent($.cookie('packet_keys')).split('-');
    }

    await wallet._keyStore.setKey(
        nearConfig.networkId,
        nearConfig.contractName,
        nearApi.KeyPair.fromString(fundingKey)
    );

    const contract = await new nearApi.Contract(
        account,
        nearConfig.contractName,
        {
            changeMethods: [
                "claim",
            ],
            sender:nearConfig.contractName
        }
    );

    await contract.claim(
        {
            "account_id":userName
        }
    ).then(res => {
        console.log(res);
        layer.closeAll();
        window.location.href = 'collect.html?type=2&key='+encodeURIComponent(packet_keys[1])
    }).catch(error=>{
        console.log(error);
        // console.log(error.toString().indexOf('啊'))
        if(error.toString().indexOf('已领取过')>=0||error.toString().indexOf('已被领取完')>=0||error.toString().indexOf('redbag before')>=0){
            window.location.href = 'collect.html?type=2&key='+encodeURIComponent(packet_keys[1])
        }else {
            layer.closeAll();
            layerBtnEvent(error)
        }
    });
}


let showClaimInfoEvent = function(pk){
    window.contract.show_claim_info(
        {
            "public_key":pk
        }
    ).then(res => {
        console.log(res);
        let jsonStr = res;
        jsonStr = jsonStr.replace("]\"", "]");
        jsonStr = jsonStr.replace(",]", "]");
        jsonStr = jsonStr.replace("\"[", "[");
        // console.log(jsonStr)
        let obj = JSON.parse(jsonStr);
        // console.log(obj)

        if(obj){
            // console.log(obj.list)
            $(".openUserName").text(obj.owner);
            $(".openUserTitle").text(obj.slogan);
        }
    }).catch(error=>{
        console.log(error);
        layerEvent(error);
    });
};


$(function(){
    // console.log(near)
    // console.log(wallet)
    // console.log(nearApi)
    // console.log(nearApi.setKey())
    // createNewContract()
    // console.log(nearApi.KeyPair.fromString())
    // console.log(wallet._keyStore.setKey())
    // console.log(wallet.getAccountId())
    // console.log(getAccount(nearConfig.contractName))
    // console.log(wallet.setKey())



    //判断登录
    // if (!window.walletAccount.isSignedIn()) {
    //     location.href = "login.html";
    // }

    //判断是否微信登陆
    function isWeiXin() {
        var ua = window.navigator.userAgent.toLowerCase();
        console.log(ua);//mozilla/5.0 (iphone; cpu iphone os 9_1 like mac os x) applewebkit/601.1.46 (khtml, like gecko)version/9.0 mobile/13b143 safari/601.1
        if (ua.match(/MicroMessenger/i) == 'micromessenger') {
            return true;
        } else {
            return false;
        }
    }
    if (isWeiXin()) {
        // alert(" 是来自微信内置浏览器")
    } else {
        // alert("不是来自微信内置浏览器")
    }

    //显示信息
    let packet_keys = getQueryVariable('packet_keys');
    if(packet_keys){
        let infoPk = decodeURIComponent(packet_keys).split('-')[1];
        showClaimInfoEvent(infoPk);
    }
    //-----open redbag-----
    $(".openBox").click(function(){

        // $.cookie('packet_keys',packet_keys);
        // let create_packet_keys;
        // if(getQueryVariable('packet_keys')){
        //     create_packet_keys = decodeURIComponent(getQueryVariable('packet_keys')).split('-');
        //     // packet_keys = $.cookie('packet_keys').split('-');
        // }else {
        //     // packet_keys = getQueryVariable('packet_keys').split('-');
        //     create_packet_keys = decodeURIComponent($.cookie('packet_keys')).split('-');
        // }
        // // console.log(packet_keys)
        //
        // createNewContract(create_packet_keys[0]);

        if (!window.walletAccount.isSignedIn()) {

            $.cookie('packet_keys',packet_keys);
            location.href = "login.html?openType=1&fundingKeys="+encodeURIComponent(decodeURIComponent(packet_keys).split('-')[0]);
        }else {
            // let packet_keys = getQueryVariable('packet_keys').split('-');
            let packet_keys;
            if(getQueryVariable('packet_keys')){
                packet_keys = decodeURIComponent(getQueryVariable('packet_keys')).split('-');
                // packet_keys = $.cookie('packet_keys').split('-');
            }else {
                // packet_keys = getQueryVariable('packet_keys').split('-');
                packet_keys = decodeURIComponent($.cookie('packet_keys')).split('-');
            }
            // console.log(packet_keys)

            createNewContract(packet_keys[0]);
        }


    })
});
