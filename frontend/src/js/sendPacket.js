
//修改红包类型
$(".amountChange").click(function(){
    let types = $(this).attr('data-type');
    if(types == '1'){
        $(this).attr('data-type',0).html('当前为普通红包，<span>改为拼手气红包</span>');
        $(".amountNameTog").text('单个总金额');
        $(".amountIconPin").hide();
        $(".totalSubmitBtn").text('发普通红包')
    }else {
        $(this).attr('data-type',1).html('当前为拼手气红包，<span>改为普通红包</span>');
        $(".amountNameTog").text('总金额');
        $(".amountIconPin").show();
        $(".totalSubmitBtn").text('发拼手气红包')
    }
});



//发红包
let requestSendRedbag = function(){
    const KeyPair = nearApi.KeyPair;
    const keyPair = KeyPair.fromRandom('ed25519');
    const newAccountPublicKey = keyPair.publicKey.toString();
    // const newAccountPublicKey = 'ed25519:2Qvv33f5oT76coyWHyG1dW3YCaygJNLYr8QEVoW3ZxS4';
    const at='00000000000000000000';
    // console.log(newAccountPublicKey)

    let allAmount = (parseFloat($("#allAmount").val()).toFixed(4)).replace('.','');
    let typeNum = $(".amountChange").attr('data-type');
    let amountLen = $("#amountLen").val();
    let sloganVal = $(".sloganTxt").val()||'万事如意，恭喜发财';
    console.log(keyPair);
    console.log(keyPair.secretKey,newAccountPublicKey)
    // console.log(newAccountPublicKey);

    $.removeCookie('packet_keys');

    if($("#allAmount").val() == ''){
        layerEvent('金额不能为空')
        return false
    }else if($("#allAmount").val() > Number($(".userBalanceTxt").text())){
        layerEvent('数额不能大于余额')
        return false
    }else if(!amountLen){
        layerEvent('请填写红包个数')
        return false
    }else {
        layer.open({
            type: 2
            ,content: '发送中'
        });

        $.cookie('packet_keys','start');
        setTimeout(function(){
            $.cookie('packet_keys',encodeURIComponent(keyPair.secretKey+'-'+newAccountPublicKey));
            console.log($.cookie('packet_keys'))
            window.contract.send_redbag(
                {
                    "public_key":newAccountPublicKey,
                    "count":Number.parseInt(amountLen),
                    "mode":Number.parseInt(typeNum),
                    "slogan":sloganVal
                },
                gas,
                allAmount+at
            ).then(res => {
                console.log(res);
                console.log('发送成功');
                // console.log($.cookie('packet_keys'));
                layer.closeAll();
                let textName = wallet.getAccountId();
                // let packet_keys = encodeURI($.cookie('packet_keys'));
                let packet_keys = $.cookie('packet_keys');
                let textareaVal = `【点击该链接即可领取${textName}发送的Near鼬红包】${base_url}openPacket.html?packet_keys=${packet_keys}`;
                $("#copyUrlTxt").val(textareaVal);
                $(".layerCopyUrlWrap").show();
            }).catch(error=>{
                console.log(error);

                layerEvent('登录信息数据过期，请重新登录');
                setTimeout(function(){
                    window.location.href = './login.html'
                },2000)
                // $.removeCookie('packet_keys')
            });
        },4000)

    }

};
$(".totalSubmitBtn").click(function(){
    let allAmount = (parseFloat($("#allAmount").val()).toFixed(4)).replace('.','');
    // console.log(parseFloat($("#allAmount").val()).toFixed(4))
    // console.log(allAmount)
    // console.log(wallet);
    // console.log(nearApi)
    requestSendRedbag();
});

//-----Storage-----
async function walletAuthKey(fundingKey){
    let keys = await localStorage.getItem("undefined_wallet_auth_key");
    return keys
}
async function keystoreTstnet(fundingKey){
    let keys = await localStorage.getItem("near-api-js:keystore:jqq.testnet:testnet");
    return keys
}

let showClaimInfoEvent = function(pk){
    window.contract.show_claim_info(
        {
            "public_key":pk
        }
    ).then(res => {
        console.log(res);
        console.log('发送成功');
        // console.log($.cookie('packet_keys'));
        layer.closeAll();
        let textName = wallet.getAccountId();
        // let packet_keys = encodeURI($.cookie('packet_keys'));
        let packet_keys = $.cookie('packet_keys');
        let textareaVal = `【点击该链接即可领取${textName}发送的Near鼬红包】${base_url}openPacket.html?packet_keys=${packet_keys}`;
        $("#copyUrlTxt").val(textareaVal);
        $(".layerCopyUrlWrap").show();
    }).catch(error=>{
        console.log(error);
        console.log('发送失败');
        console.log($.cookie('packet_keys'));
        if($.cookie('packet_keys')){
            layerBtnEvent('发送失败或拒绝');
        }else {
            layer.closeAll();
        }
    });
};

$(function(){
    console.log(wallet)
    console.log(window)
    console.log(near)

    //-----loading-----
    layer.open({
        type: 2
    });

    //判断登录
    if (!window.walletAccount.isSignedIn()) {
        location.href = "login.html";
    }

    //发送成功
    let sendPacketEvent = function(){
        let sendType = getQueryVariable('sendType');
        // console.log(sendType)
        if(sendType&&sendType == 'success'){
            console.log('发送成功');
            // console.log($.cookie('packet_keys'));
            layer.closeAll();
            let textName = wallet.getAccountId();
            // let packet_keys = encodeURI($.cookie('packet_keys'));
            let packet_keys = $.cookie('packet_keys');
            let textareaVal = `【点击该链接即可领取${textName}发送的Near鼬红包】${base_url}openPacket.html?packet_keys=${packet_keys}`;
            $("#copyUrlTxt").val(textareaVal);
            $(".layerCopyUrlWrap").show();
        }else {
            console.log('发送失败');
            console.log($.cookie('packet_keys'));
            if($.cookie('packet_keys')){
                layerBtnEvent('发送失败');
            }else {
                layer.closeAll();
            }
        }
    };

    //余额
    wallet.account().state().then(data => {
        console.log(data);
        // console.log(data.amount/radix);
        let amount = data.amount/radix;
        let balances = Math.floor(amount * 10000) / 10000;
        $(".userBalanceTxt").text(balances);
        // console.log(Number(balances))
        // layer.closeAll();
        // console.log(window.near)
        // console.log(window.near.connection)
        // console.log(window.wallet.requestSignTransactions)
        // console.log(window.walletAccount.signAndSendTransaction())
        if($.cookie('packet_keys')&&decodeURIComponent($.cookie('packet_keys')).indexOf('-') >=0){
            console.log($.cookie('packet_keys'))
            let packetKeys = decodeURIComponent($.cookie('packet_keys')).split('-');
            showClaimInfoEvent(packetKeys[1])
        }else {
            layer.closeAll();
        }
        // sendPacketEvent()
    });

    //名称
    const userName=wallet.getAccountId();
    $(".userName").text(userName);



    //退出
    //login out
    $(".loginOut").click(function(){
        layer.open({
            content: 'sign out?'
            ,btn:['YES','NO']
            ,yes: function(){
                window.walletAccount.signOut();
                layer.closeAll();
                $.removeCookie('packet_keys');
                location.href='login.html';
            }
        });
    });

    //-----copy------
    var clipboard = new ClipboardJS('.copyUrlBtn');
    clipboard.on('success', function(e){
        // console.log(e);
        console.log('复制成功');
        layerEvent('复制成功')
    });
    clipboard.on('error', function(e){
        console.log('复制失败，请长按复制');
        layerEvent('复制失败，请长按复制')
    });
    $(".closeBtn").click(function(e){
        $(".layerCopyUrlWrap").hide()
    });
    // $(".layerCopyUrlBox").click(function(e){
    //     e.stopPropagation();
    // })
});
