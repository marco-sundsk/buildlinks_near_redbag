
//-----connection-----
function getAccount(accountId) {
    // console.log('this.connection',window.near.connection)
    return new nearApi.Account(window.near.connection, accountId)
}

//-----revoke-----
let collectTotalListEvent = function(key){
    layer.open({
        content: '您确定撤回红包吗？'
        ,btn: ['确定', '取消']
        ,yes: function(index){
            layer.open({
                type: 2
                ,content: '正在撤回'
            });
            window.contract.revoke(
                {
                    "public_key":key
                }
            ).then(res => {
                // console.log(res);
                window.location.href = 'packetHistory.html'
                // requestShowRedBag()
            }).catch(error=>{
                // console.log(error);
                layerBtnEvent(error)
            });
        }
    });

};

//-----show_claim_info-----
let showClaimInfoEvent = function(pk){
    layer.open({
        type: 2
        ,content: ''
    });
    window.contract.show_claim_info(
        {
            "public_key":pk
        }
    ).then(res => {
        console.log(res)

        let jsonStr = res;
        jsonStr = jsonStr.replace("]\"", "]");
        jsonStr = jsonStr.replace(",]", "]");
        jsonStr = jsonStr.replace("\"[", "[");
        // console.log(jsonStr)
        let obj = JSON.parse(jsonStr);
        // console.log(obj)

        if(obj){
            layer.closeAll();
            // console.log(obj.list)
            let alreadyReceiveLen = obj.list;
            let count = obj.count;
            let modeTxt = obj.mode == 1?'的拼手气红包':'的普通红包';
            let sloganTxt = obj.slogan?obj.slogan:'恭喜发财，万事如意';
            let defaultHtml = `<div class="collectTotalListTop">领取${alreadyReceiveLen.length}/${count}个</div><div class="collectTotalListBox"></div>`;
            $("#modeTxt").text(modeTxt);
            $("#totalName").text(obj.owner);
            $(".moneyTitle").text(sloganTxt);
            $(".collectTotalListWrap").html(defaultHtml);
            let type = getQueryVariable('type');
            if(parseInt(obj.remaining) !== 0&&type == 1){
                $(".packetHistoryBack").show()
            }
            if(alreadyReceiveLen.length>0){
                let allAmount = 0;
                for(let i in alreadyReceiveLen){ //(parseInt(res)/radix
                    allAmount = Number(allAmount) + Number(alreadyReceiveLen[i].amount)/radix;
                    let li = '<div class="collectTotalList" data-id="'+alreadyReceiveLen[i].account+'">\n' +
                        '            <div class="collectTotalIcon"></div>\n' +
                        '            <div class="collectTotalDetail">\n' +
                        '                <div class="collectTotalName">'+alreadyReceiveLen[i].account+'</div>\n' +
                        // '                <div class="collectTotalTime">12-24 18:00</div>\n' +
                        '            </div>\n' +
                        '            <div class="collectTotalManyIcon"></div>\n' +
                        '            <div class="collectTotalMany">'+Number(Number(alreadyReceiveLen[i].amount)/radix).toFixed(2)+'</div>\n' +
                        '        </div>';
                    $(".collectTotalListBox").prepend(li)
                }
                $(".moneyNumber span").text(allAmount.toFixed(2))
            }
        }
    }).catch(error=>{
        console.log(error);
        layer.closeAll();
        layerBtnEvent(error)
    });
};


$(function(){
    // console.log(nearApi.Account)
    //  console.log(window.wallet)
    //  console.log(window.near.connection)
    // console.log('start',user)
    // console.log(gas)
    // console.log(newAccountPublicKey)
    // console.log(wallet.getAccountId())

    let infoPK = getQueryVariable('key')||$.cookie('packet_keys')?decodeURIComponent($.cookie('packet_keys')).split('-')[1]:'';
    // let infoPK = getQueryVariable('key');
    let key = decodeURIComponent(infoPK);
    // console.log(key)
    if(key){
        showClaimInfoEvent(key)
    }

    // -----revoke-----
    $(".packetHistoryBack").click(function(){
        let key = getQueryVariable('key');
        collectTotalListEvent(key)
    })
});
