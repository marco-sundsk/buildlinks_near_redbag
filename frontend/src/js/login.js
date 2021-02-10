
let openUrl = './sendPacket.html';
let replaceVal = 'sendPacket.html';
async function doWork() {
    // Based on whether you've authorized, checking which flow we should go.
    let successUrl=window.location.href.replace('login.html',replaceVal);
    if (!window.walletAccount.isSignedIn()) {
        window.walletAccount.requestSignIn(
            // The contract name that would be authorized to be called by the user's account.
            // window.nearConfig.contractName,
            window.nearConfig.contractName,
            // This is the app name. It can be anything.
            "Welcome to NEAR",
            successUrl,
        );
    } else {
        location.href = openUrl;
    }
}
$(".oldUser").click(() => {
    doWork()
});
$(".createUser").click(()=>{
    let fundingContracts = window.nearConfig.contractName;
    let fundingKeys = decodeURIComponent(getQueryVariable('fundingKeys'));
    // console.log(fundingContracts,fundingKeys)
    if(fundingKeys&&fundingContracts){
        layer.open({
            type: 2
            ,content: '加载中'
        });
        window.location.href = `${create_url}create/${fundingContracts}/${fundingKeys}?create=1&success_url=${base_url}openPacket.html`;

    }
});

window.onload=function(){
    // console.log(window.walletAccount)
    if(getQueryVariable('openType')&&Number(getQueryVariable('openType')) == 1){
        replaceVal = 'openPacket.html';
        $(".oldUserTog").css('paddingTop',0).find('button').text('已有账号抢');
        $(".createTog").show();
    }else if (window.walletAccount.isSignedIn()) {
        window.location.href = openUrl;
    }else if(!window.walletAccount.isSignedIn()){
        $(".oldUserTog").css('paddingTop','.3rem').find('button').text('已有账号登录');
        $(".createTog").hide();
    }
};
