async function init() {
    let rustApp = null

    try{
        rustApp = await import('../pkg')
    }catch(e){
        console.error(e)
        return;
    }

    const input = document.getElementById('upload')
    const fileReader = new FileReader()

    fileReader.onloadend = () => {
        const base64 = fileReader.result.replace(
            /^data:image\/(png|jpeg|jpg);base64,/, ''
        )
        let img_data_url = rustApp.grayscale(base64)
        let greyScaledImage = document.getElementById('new-img')
        greyScaledImage.style.width = "800px"
        greyScaledImage.style.height = "600px"
        greyScaledImage.setAttribute(
            'src',
            img_data_url
        )

    }

    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0])   
    })

}

init();
