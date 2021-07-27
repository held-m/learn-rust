function changeLang(lang){

    var styleBorder = "#698277 solid 1px";

    switch (lang) {
        case 'en':
            projOnline.innerText = "Projects online:";
            iExperiance.innerText = "My experience:";
            language.innerText = "Languages:";
            contact.innerText = "Contacts";
            city.innerText = "Regensburg, Germany";
            removeUnderLineLang();
            langSelect[1].style.borderBottom = styleBorder;
            langSelect[1].style.borderTop = styleBorder;
            break;
        case 'ru':
            projOnline.innerText = "Проекты, которые в онлайн:";
            iExperiance.innerText = "Что я использовал:";
            language.innerText = "Языки:";
            contact.innerText = "Контакты";
            city.innerText = "Регенсбург, Германия";
            removeUnderLineLang();
            langSelect[0].style.borderBottom = styleBorder;
            langSelect[0].style.borderTop = styleBorder;
            break;
        case 'de':
            projOnline.innerText = "Projekte online:";
            iExperiance.innerText = "Meine Erfahrung:";
            language.innerText = "Sprache:";
            contact.innerText = "Kontakt";
            city.innerText = "Regensburg, Deutschland";
            removeUnderLineLang();
            langSelect[2].style.borderBottom = styleBorder;
            langSelect[2].style.borderTop = styleBorder;
            break;
        default:
            projOnline.innerText = "Projects online:";
            iExperiance.innerText = "My experience:";
            language.innerText = "Languages:";
            contact.innerText = "Contacts";
            city.innerText = "Regensburg, Germany";
            removeUnderLineLang();
            langSelect[1].style.borderBottom = styleBorder;
            langSelect[1].style.borderTop = styleBorder;
    }
}
function removeUnderLineLang() {
    for (var i = 0; i < langSelect.length; i++) {
        langSelect[i].style.borderBottom = "";
        langSelect[i].style.borderTop = "";
    }
}