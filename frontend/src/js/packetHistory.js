
//-----to collect-----
let collectTotalListEvent = function(pk){
   $(".collectTotalList").click(function(){
       let key = $(this).attr('data-key');
       // console.log(key);
       window.location.href = 'collect.html?type=1&key='+key
   })
};

//切换未领取完
let togReceive = function(self){
    if(!$(self).find('button').hasClass('on')){
        $(self).find('button').addClass('on');
        $(".collectTotalList").each(function(index,item){
            // console.log(index,item)
            let isReceive = $(item).attr('data-is');
            if(isReceive == 1){
                $(item).hide()
            }
        });
    }else {
        $(self).find('button').removeClass('on');
        $(".collectTotalList").each(function(index,item){
            $(item).show()
        });
    }
};

//-----show_claim_info-----
let allAmountMax = 0;
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
            let alreadyReceiveLen = obj.list;
            let allAmount = 0;
            if(alreadyReceiveLen.length>0){
                for(let i in alreadyReceiveLen){
                    allAmount += Number(alreadyReceiveLen[i].amount)
                }
            }
            allAmountMax = Number(allAmountMax) + Number(allAmount);
            $(".moneyNumber span").text(Number(allAmountMax/radix).toFixed(2));
            $("#totalName").text(obj.owner);

            let collectTotalName = wallet.getAccountId();
            let receiveOver = parseInt(obj.remaining) == 0?'已领完':'未领完';
            let isReceiveOver = parseInt(obj.remaining) == 0?1:0;
            let li = '<div class="collectTotalList" data-is="'+isReceiveOver+'" data-key="'+pk+'">\n' +
                '            <div class="collectTotalDetail">\n' +
                '                <div class="collectTotalName">'+collectTotalName+'</div>\n' +
                // '                <div class="collectTotalTime">12-24 18:00</div>\n' +
                '            </div>\n' +
                '            <div class="collectTotalManyWrap">\n' +
                '                <div class="collectTotalManyTop">\n' +
                '                    <div class="collectTotalManyIcon"></div>\n' +
                '                    <div class="collectTotalMany">'+Number(allAmount/radix).toFixed(2)+'</div>\n' +
                '                </div>\n' +
                '                <div class="collectTotalManyBottom">\n' +
                '                    <span class="receiveDetailTxt">'+receiveOver+'</span><span class="receiveDetail">'+alreadyReceiveLen.length+'/'+obj.count+'</span><span>个</span>\n' +
                '                </div>\n' +
                '            </div>\n' +
                '        </div>'
            $(".collectTotalListBox").prepend(li);
            collectTotalListEvent()
        }
    }).catch(error=>{
        console.log(error);
    });
};

//-----show_redbag-----
let requestShowRedBag = function(){
    const userName=wallet.getAccountId();
    window.contract.show_redbag(
        {
            "account_id":userName
        }
    ).then(res => {
        console.log(res)
        // console.log(typeof res)
        // console.log(res.length)

        if(res.length>0){
            $("#sendPacketLen").text(res.length);
            let defaultHtml = '' +
                '<div class="collectTotalListTop" onclick="togReceive(this)"><button></button><span>只显示未领完红包</span></div>' +
                '<div class="collectTotalListBox"></div>';
            $(".collectTotalListWrap").html(defaultHtml);

            for(let i in res){
                showClaimInfoEvent(res[i])
            }
        }
    }).catch(error=>{
        console.log(error);
        layerEvent(error);
    });
};


$(function(){
    // console.log(nearApi.Account)
    //  console.log(window.wallet)
    //  console.log(window.near.connection)
    // console.log('start',user)
    // console.log(gas)
    // console.log(newAccountPublicKey)

    requestShowRedBag();

    // showClaimInfoEvent('ed25519:8gUcDhJqKWEydhqfaWbZVYVsUabvTJxGv7CrGUA5u6gb');
    // showClaimInfoEvent('ed25519:GV5YKVgA6Gn7PwndQaC88aoBe9Qp2z7t6qa1EKawdR3H');

    // $(".collectTop").click(function(){
        // let sk = '3bAdrzth3WtDAiXz6h18hRgatzoujbuSCmieawgwicVTPZNuTvZLyLceeKg1VW2x28ZbwssVSXGwZQFgmztvPfjq';

        // createNewContract(sk);
        // requestShowRedBag()
    // });
});
