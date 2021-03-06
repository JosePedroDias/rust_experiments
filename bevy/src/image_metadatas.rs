use super::random::{get_usize, setup_random_seed};
use bevy::math::Vec2;

#[derive(Clone, Debug)]
pub struct ImageMetadata {
    pub file_name: String,
    pub dims: Vec2,
    pub title: String,
    pub author: String,
    pub url: String,
}

pub fn select_image(image_index: Option<usize>) -> ImageMetadata {
    let images: Vec<ImageMetadata> = vec![
        ImageMetadata {
            file_name: String::from("13429106963_a074dbdbcd_k"),
            dims: Vec2::new(2048., 1365.),
            title: String::from("IMG_6087 (Berlin)"),
            author: String::from("Andie Sticks"),
            url: String::from("https://www.flickr.com/photos/andiesticks/13429106963/in/photolist-msFFCX-EJGSF7-msFaua-54oqYa-msHn6o-msFvM4-msEAGt-msF4Jc-3hMAC-msF8Ev-msF8qk-54orEV-msHbPm-msGPKs-msEMfc-rMcbRC-21asuqi-54oqb2-54os4B-msFAJR-wez8o4-G5ac3S-2iYZ6bx-EUK7yN-wLs6To-2jvfYJC-P6Ph4Y-2hphkot-4XuM4d-25Gwrqs-2h6qgPd-5cSrV3-28J4Y9f-FhSsmU-G7t7yH-2ia8QyP-FNcYG7-dCXP13-WqjFM3-EpgUif-EFoG97-2hJTb8A-Fyh9rV-EJGSYS-G7t8ea-w4Fakr-GE6mMR-Fi49Gc-GdjqQt-FhSsaw"),
        },
        ImageMetadata {
            file_name: String::from("1478451436_e52fe4f2d3_o"),
            dims: Vec2::new(1024., 768.),
            title: String::from("Landmark Tower"),
            author: String::from("sigusr0"),
            url: String::from("https://www.flickr.com/photos/sigusr0/1478451436/in/photolist-3fDs71-hko5Jr-2cdA9he-2jDYm6X-GaH7uJ-G5abNU-EBSef2-ERRXQX-6mTMmm-KYRKhR-Fi49Wa-dCSpL8-wjwmrd-rZi4r4-EiT6KV-2jHAp9r-2RUn5-FvY4Hf-DU1R5Z-Eiojgi-VJksfJ-uYVND9-LXvEva-FhSvUA-DiMG56-EDCCD1-FvY4Fb-GrVeak-EJUo5V-zsAnhU-DTFdkA-2k4xDdd-2ipGLKR-2dnUbkS-7sudkX-Ej6nnN-EJGRQj-7HSRix-24Pjtnd-SskZnM-WuzcdW-Ah8omN-iiuSyN-DPjWWy-WuzcTU-6LZrn8-fSwUA-Wuzacb-yNmbW5-Fyh988"),
        },
        ImageMetadata {
            file_name: String::from("17461775402_859930f2f9_k"),
            dims: Vec2::new(2048., 1363.),
            title: String::from("(Really) Big Ben"),
            author: String::from("Phil Dolby"),
            url: String::from("https://www.flickr.com/photos/126654539@N08/17461775402/in/photolist-sB3at9-EP66aQ-25dDoAR-AiessS-VMy5QV-eb9hXv-4iHYE8-kR1jGN-2yRhrM-4EwcUY-FhSvZf-EHce1P-EpgUmm-3fDs71-hko5Jr-2cdA9he-2jDYm6X-GaH7uJ-G5abNU-EBSef2-ERRXQX-6mTMmm-KYRKhR-Fi49Wa-dCSpL8-wjwmrd-rZi4r4-EiT6KV-2jHAp9r-2RUn5-FvY4Hf-DU1R5Z-Eiojgi-VJksfJ-uYVND9-LXvEva-FhSvUA-DiMG56-EDCCD1-FvY4Fb-GrVeak-EJUo5V-zsAnhU-DTFdkA-2k4xDdd-2ipGLKR-2dnUbkS-7sudkX-Ej6nnN-EJGRQj"),
        },
        ImageMetadata {
            file_name: String::from("21527914408_acebd6c1bd_k"),
            dims: Vec2::new(2048., 1091.),
            title: String::from("Chicago Landmarks"),
            author: String::from("anax44"),
            url: String::from("https://www.flickr.com/photos/aneil4lom/21527914408/in/photolist-yNmbW5-Fyh988-VWJJ5A-VZ24ia-UKsgca-NvRras-EpgWgd-2bqqeTN-VP28mu-GcZP1D-EJJELd-eWG6HZ-JphzhX-2igsFGB-T2rPK3-VZ27qx-G79ucR-2cC1zWG-UKsfwx-VNN2ke-TveoL5-VZ25dB-NWeyn4-znueyk-Cmh3Ms-gBcmcC-Wuzdyw-DNBcXE-8nsAMu-VZ25Y4-VZ26Wg-8rbfuk-znpC5y-AjrfWt-Wuzbwf-VMy57v-Ajo2gP-DgtpaU-UKsfdX-FhJA92-PMqyCw-VVwfaC-MpA1tb-ERoWk6-UKseUa-Ah8n7o-58dtQN-UKsefK-DNFUuy-EdyG46"),
        },
        ImageMetadata {
            file_name: String::from("21799002712_960191795d_k"),
            dims: Vec2::new(2048., 1371.),
            title: String::from("_DSC0672"),
            author: String::from("Jordan Uhl"),
            url: String::from("https://www.flickr.com/photos/jordanuhl/21799002712/in/photolist-ihmKH-4HRT3S-sykX-4Ds8DG-vvBsj2-776Zei-2CCfk-4DnTnT-9TEYRH-snLUXQ-9THNhW-9JPU1R-e5G5C-2iPihG-2bvmt-55Jzeh-9iD5ks-sEfiNd-9THNdj-2isggf-4UAaAH-rKrpep-4944E2-77aVU5-9THNyE-5henQp-7vD423-9TEYUi-eDPm7-KZZvhi-7vL84D-b8GpyB-fXq6a-91ibxU-WedR15-776ZGV-77aWmW-zdiA3m-nKnMkB-4UEoxu-3QrHpF-4UA8TF-91f9cp-68ZxyP-7vPVoA-66G5TV-ehdBFo-7vPVGS-5hiWWY-4HRTb1"),
        },
        ImageMetadata {
            file_name: String::from("23364494180_b99e33a74d_k"),
            dims: Vec2::new(2048., 1135.),
            title: String::from("Over The Hill"),
            author: String::from("Phil Dolby"),
            url: String::from("https://www.flickr.com/photos/126654539@N08/23364494180/in/photolist-BAD97y-5PUibb-u1KpY6-6C58PB-9TEZbZ-9TEYFg-Lk9z2R-4DnSSH-J2NmfD-5ub67U-4Ds9GG-ihmKH-4HRT3S-sykX-4Ds8DG-vvBsj2-776Zei-2CCfk-4DnTnT-9TEYRH-snLUXQ-9THNhW-9JPU1R-e5G5C-2iPihG-2bvmt-55Jzeh-9iD5ks-sEfiNd-9THNdj-2isggf-4UAaAH-rKrpep-4944E2-77aVU5-9THNyE-5henQp-7vD423-9TEYUi-eDPm7-KZZvhi-7vL84D-b8GpyB-fXq6a-91ibxU-WedR15-776ZGV-77aWmW-zdiA3m-nKnMkB"),
        },
        ImageMetadata {
            file_name: String::from("24874850997_7a5cc52ad5_k"),
            dims: Vec2::new(2048., 1212.),
            title: String::from("Chapel of The Holy Cross (1956)"),
            author: String::from("prayitnophotography"),
            url: String::from("https://www.flickr.com/photos/prayitnophotography/24874850997/in/photolist-DU77zk-zewYkx-GdaaZz-zsdVPP-yz4uoY-zuGcWK-whs2Xq-7kLB3A-EP67tw-ywLaWU-zcfJkS-zcaJbW-ehkJzm-A19BPP-z3koj8-ze3s7x-zuGgnH-FhHhus-86tQkL-zKDJw8-GdadbZ-zwVnvX-ehkdJJ-FhTTnT-yz286G-zs94qQ-zdW8Sj-zrvLEf-e6V95j-yMSWJg-zepDgU-KFv31N-oKE8qw-yz5YBu-zeAPwT-zeAPDg-oKEqH4-p39BGK-p37RQL-zwYP2k-G5adx5-FhSwUG-yz5ySq-Fi4bge-zv2K6w-G7t8Vk-zwXHXe-yzdSsD-zw7QUn-zwYP6Z"),
        },
        ImageMetadata {
            file_name: String::from("25688225598_63d3e2ed15_o"),
            dims: Vec2::new(2000., 1333.),
            title: String::from("Chureito Pagoda - Fujiyoshida-shi, Japan"),
            author: String::from("Giuseppe Milo"),
            url: String::from("https://www.flickr.com/photos/giuseppemilo/25688225598/in/photolist-F8YSvm-Ta9dyu-FhGVpZ-TyUJzM-GaH7zo-52Ean3-2kDZdnk-dW18HH-FhxWSu-G79w8z-2ehGXwP-HHtsRU-NV5drS-SsCfRu-7be8Vr-2hZ6jj-2kEt6iE-Va9HaX-2jGT7AJ-2iwWLbQ-CHHa6B-UUJLz1-tmSBnm-2hsLzDA-be8sTH-2jXUVJJ-22iv4Y3-2jLcadB-8CV8DU-ozZBVt-Tvep5w-9GrKbT-7xqeRc-xxYg28-2gXsmuo-rK4sgT-2dHPnpC-GcZRFD-G79w8K-4TbEZ9-FhxWqN-FMTvMS-5ph9St-6tiZ8a-TwrTz7-8qXhNf-9jdx5J-91TKss-5rDhuN-5uXbuz"),
        },
        ImageMetadata {
            file_name: String::from("26570523185_e33f8c30c4_k"),
            dims: Vec2::new(2048., 1365.),
            title: String::from("Edith - E"),
            author: String::from("Mark Gunn"),
            url: String::from("https://www.flickr.com/photos/mark-gunn/26570523185/in/photolist-GtWSQH-9TEYPk-Kb44D7-4944rD-krmH1u-77AtmT-91iggW-9TEYX6-2hQxmo-4HMCqH-9iD5nA-4ewvQy-2is4voQ-2isi31-2CCfj-aCc62m-2eyBLm6-9TEYL6-4UEuqY-4DnRvT-zPTyw-ehdE9S-EoFkjU-92Pyy4-6osfY-4FfqoL-6ENfNE-BAD97y-5PUibb-u1KpY6-6C58PB-9TEZbZ-9TEYFg-Lk9z2R-4DnSSH-J2NmfD-5ub67U-4Ds9GG-ihmKH-4HRT3S-sykX-4Ds8DG-vvBsj2-776Zei-2CCfk-4DnTnT-9TEYRH-snLUXQ-9THNhW-9JPU1R"),
        },
        ImageMetadata {
            file_name: String::from("26685343067_a447f3feaf_k"),
            dims: Vec2::new(2047., 1346.),
            title: String::from("Peace Bridge. Calgary"),
            author: String::from("Bernard Spragg. NZ"),
            url: String::from("https://www.flickr.com/photos/volvob12b/26685343067/in/photolist-GE6mMR-Fi49Gc-GdjqQt-FhSsaw-7JMfSe-V4YTLV-29ARvfd-EhVeLR-Fm89DK-2gVCfRS-2iUFHvZ-6kodM1-EpgVBY-FNcXhJ-29xemj7-EP65j1-DTFdLW-G7t4kZ-FhSs75-msFxrZ-msFLNM-msF698-54sEE9-msHq2o-2kuaPvk-r843zK-Et1XUq-54or42-msHquC-msFkrB-msFDt6-msF4fB-msF6V6-msGc5c-msFsAB-msFgKX-PvfYiN-3DKTD-msGpDb-msGtSw-msFATr-msFiY4-msGKaW-r84c2p-msF4u4-24gKwCx-54oqWB-54sEVh-54sEQ3-54oqVt"),
        },
        ImageMetadata {
            file_name: String::from("27379962206_ef039cfc32_k"),
            dims: Vec2::new(2048., 1365.),
            title: String::from("Triomphe"),
            author: String::from("Jayphen"),
            url: String::from("https://www.flickr.com/photos/jayphen/27379962206/in/photolist-HHtsRU-NV5drS-SsCfRu-7be8Vr-2hZ6jj-2kEt6iE-Va9HaX-2jGT7AJ-2iwWLbQ-CHHa6B-UUJLz1-tmSBnm-2hsLzDA-be8sTH-2jXUVJJ-22iv4Y3-2jLcadB-8CV8DU-ozZBVt-Tvep5w-9GrKbT-7xqeRc-xxYg28-2gXsmuo-rK4sgT-2dHPnpC-GcZRFD-G79w8K-4TbEZ9-FhxWqN-FMTvMS-5ph9St-6tiZ8a-TwrTz7-8qXhNf-9jdx5J-91TKss-5rDhuN-5uXbuz-TLoxfK-66E17B-GaH7W5-91TKnd-bLsxw-Ta9e3f-9GrQnB-Ta9exJ-9GuDc1-ehhRPq-X8xQCJ"),
        },
        ImageMetadata {
            file_name: String::from("29140894326_1e60dd4a6e_k"),
            dims: Vec2::new(2048., 1365.),
            title: String::from("Singapore - City & Merlion"),
            author: String::from("south"),
            url: String::from("https://www.flickr.com/photos/southtopia/29140894326/in/photolist-Lp5GCw-zBeX2z-eXqrm2-MAGEn5-4UzU68-MJ1wwc-2grZm46-GtWSQH-9TEYPk-Kb44D7-4944rD-krmH1u-77AtmT-91iggW-9TEYX6-2hQxmo-4HMCqH-9iD5nA-4ewvQy-2is4voQ-2isi31-2CCfj-aCc62m-2eyBLm6-9TEYL6-4UEuqY-4DnRvT-zPTyw-ehdE9S-EoFkjU-92Pyy4-6osfY-4FfqoL-6ENfNE-BAD97y-5PUibb-u1KpY6-6C58PB-9TEZbZ-9TEYFg-Lk9z2R-4DnSSH-J2NmfD-5ub67U-4Ds9GG-ihmKH-4HRT3S-sykX-4Ds8DG-vvBsj2"),
        },
        ImageMetadata {
            file_name: String::from("2953055417_58092c2faf_k"),
            dims: Vec2::new(2047., 1149.),
            title: String::from("Columbus Tower / San Francisco Flat Iron Building"),
            author: String::from("Scott Schiller"),
            url: String::from("https://www.flickr.com/photos/schill/2953055417/in/photolist-5uXbuz-TLoxfK-66E17B-GaH7W5-91TKnd-bLsxw-Ta9e3f-9GrQnB-Ta9exJ-9GuDc1-ehhRPq-X8xQCJ-JvnWM-5jSU9D-Ta9cqs-yroGzF-9THNPE-2hB5QyV-Ta9ehU-9GrLqz-UUJPvG-9GuCDo-QnGZFf-btoUHp-9GrJkH-9GrJTR-UUJEo3-9GuEPG-91TKgY-5hML7Y-VVz9ZE-TwrQD7-Ta9djS-9GuJNQ-9GrPHK-TwrPm7-9GrQ6B-9GrKjR-TyUMSi-9GuFno-9GuCUb-TyUPa8-9GrP5c-9GrLh6-9GuGSh-9GuG6q-TLoAf2-9GuH7o-9GrKAR-9JSGPh"),
        },
        ImageMetadata {
            file_name: String::from("30011450063_fc68d29177_k"),
            dims: Vec2::new(2048., 1367.),
            title: String::from("Bastei Bridge in Saxon Switzerland"),
            author: String::from("Bernd Thaller"),
            url: String::from("https://www.flickr.com/photos/bernd_thaller/30011450063/in/photolist-MJ1wwc-2grZm46-GtWSQH-9TEYPk-Kb44D7-4944rD-krmH1u-77AtmT-91iggW-9TEYX6-2hQxmo-4HMCqH-9iD5nA-4ewvQy-2is4voQ-2isi31-2CCfj-aCc62m-2eyBLm6-9TEYL6-4UEuqY-4DnRvT-zPTyw-ehdE9S-EoFkjU-92Pyy4-6osfY-4FfqoL-6ENfNE-BAD97y-5PUibb-u1KpY6-6C58PB-9TEZbZ-9TEYFg-Lk9z2R-4DnSSH-J2NmfD-5ub67U-4Ds9GG-ihmKH-4HRT3S-sykX-4Ds8DG-vvBsj2-776Zei-2CCfk-4DnTnT-9TEYRH-snLUXQ"),
        },
        ImageMetadata {
            file_name: String::from("3720835642_a4f34dedfd_o"),
            dims: Vec2::new(1504., 1000.),
            title: String::from("Colisseum"),
            author: String::from("Kat Martin"),
            url: String::from("https://www.flickr.com/photos/kat_martin/3720835642/in/photolist-6ENfNE-BAD97y-5PUibb-u1KpY6-6C58PB-9TEZbZ-9TEYFg-Lk9z2R-4DnSSH-J2NmfD-5ub67U-4Ds9GG-ihmKH-4HRT3S-sykX-4Ds8DG-vvBsj2-776Zei-2CCfk-4DnTnT-9TEYRH-snLUXQ-9THNhW-9JPU1R-e5G5C-2iPihG-2bvmt-55Jzeh-9iD5ks-sEfiNd-9THNdj-2isggf-4UAaAH-rKrpep-4944E2-77aVU5-9THNyE-5henQp-7vD423-9TEYUi-eDPm7-KZZvhi-7vL84D-b8GpyB-fXq6a-91ibxU-WedR15-776ZGV-77aWmW-zdiA3m"),
        },
        ImageMetadata {
            file_name: String::from("42018982531_2abd55c396_o"),
            dims: Vec2::new(2048., 1200.),
            title: String::from("Landmark 81, 461 mètre. Ho Chi Minh City, Vietnam"),
            author: String::from("Lee Nguyen"),
            url: String::from("https://www.flickr.com/photos/145283084@N04/42018982531/in/photolist-2725gA4-EpgVQJ-p39tNt-Fyh9YB-p37HMb-yMTDfM-FvY4DY-2gBad8h-oKDYwu-yz5yao-oKEryd-yMKPf3-oKEr9A-p2Tgzg-zbZgAk-ehkdbq-oKEqPX-Q9HTUE-oKDMb2-DPEEje-zuZeyj-zs944Y-zGrkyw-4tjMkD-5ejdDV-zeuvmL-56SYQh-zsfoot-HUAdt-FN3KVo-Gdad8n-EpgWQE-zseDeT-5eoBj5-EjVEJL-zwUkGt-4toPSN-zHDPbb-zJGz9z-EAzQws-zepzvj-zewXUn-Gdadhk-G7iSBT-zs6T1C-Ed4GnR-zw4Wbv-DPaGjz-FMTuNY-rbsb76"),
        },
        ImageMetadata {
            file_name: String::from("4229517552_7269153fcf_o"),
            dims: Vec2::new(2816., 2061.),
            title: String::from("Hallgrímskirkja - an architectural landmark in Reykjavik, Iceland"),
            author: String::from("O Palsson"),
            url: String::from("https://www.flickr.com/photos/opalsson/4229517552/in/photolist-7rKodq-zsgZpi-2jzgvW9-GaSoiQ-FhTSPD-fjBZrF-ztvjYT-znj7Mf-G7t9dp-2cSHSDH-V5nH4-zunufe-zsbFZ8-7CjqHd-FhHe7f-p37K5b-p17RMA-a7heg3-DNBcQL-DPaFBc-yMGKsQ-o9Prpk-PoDtK1-G7iS2K-e6V8jC-HWiHTf-EFoFc7-e6V8Vw-EJeEtY-2jzgwjo-e6V8yQ-4tnDGE-GaSoEw-FN3NY7-ehkdE1-p37zfS-EcQWgP-q7Ux6P-DP2EDg-oKEhoZ-a7cVMX-p2T8cp-FhTUEn-FhTVjD-EpgWd7-ehdAoh-GdacY4-z6Lgze-FN3PK7-yMEvAL"),
        },
        ImageMetadata {
            file_name: String::from("42406539775_22a02e7504_o"),
            dims: Vec2::new(2560., 1920.),
            title: String::from("Uniao Fraterna Building"),
            author: String::from("Diego Torres Silvestre"),
            url: String::from("https://www.flickr.com/photos/3336/42406539775/in/photolist-27BjARv-8npv2R-pf3XAs-5eoBDS-znmqwE-MiQ97R-a7ebvF-EJGTe1-Fyh9JP-A2X8KF-7syadN-8nsC6f-EEVK4j-5TXKSY-EEVJUb-4gyabp-zZjyJ-EhVfpp-D34LXa-A2KPrC-b2c64v-ywqUJh-p17RLy-oH7uGh-DNFUG7-VMy4AR-G79vPD-UKsfQD-VVwfs1-UKsh4k-VMy3vp-VZ252z-zuJKtZ-G4Qxss-EcJpwK-5ZNwe8-PoDvdG-EcJo2k-FN3LK9-zcb1Cb-DPEyxV-EjhEDu-EcQWX8-G7iSJX-zJLWik-9Htvk2-ywQLfL-563v1B-DP2Bbz-DiMETD"),
        },
        ImageMetadata {
            file_name: String::from("42748712482_da722406f4_o"),
            dims: Vec2::new(2592., 1944.),
            title: String::from("Putrajaya Landmark"),
            author: String::from("Przemek P"),
            url: String::from("https://www.flickr.com/photos/globalquiz/42748712482/in/photolist-288yjSW-oKDNQV-yyYCHd-zsojKU-2jzfPzm-EJ1Wfo-znmqfN-p17CSC-D2kQPQ-4DnLNg-dXhAJX-CDSnh1-8WPtxF-oKE2mh-u4YD8X-7gQgFV-p2Tjdn-RBjA4F-YtKqaF-EHX7SU-p2Tjic-28qiRVq-MXwDpT-q6CDoU-p37Lcm-8Jtrbu-dQsgxy-HrN1Pt-yA8sg-p17Luw-oKDPT6-8nXkKU-s5tbSa-zHEwaL-oKEjCj-a7eAKe-zs92Gm-Ej6tgw-DNWWhv-7rQRtn-6cgbDh-2iAiCva-2jzgoQL-EcESXp-dZ7bjU-ouphb2-DNT5Gp-DNT5qc-2kfiaTk-dY2NgF"),
        },
        ImageMetadata {
            file_name: String::from("4357937902_58b8753148_k"),
            dims: Vec2::new(2047., 1270.),
            title: String::from("St. Paul ruins, Macau. China"),
            author: String::from("Sherpas 428"),
            url: String::from("https://www.flickr.com/photos/sherpas428/4357937902/in/photolist-7D6z7o-rHqcVd-ehiwfW-ehhUvC-7UbDFT-s4H17r-5henuR-EZExU-fnUKen-cdi27j-4UAcHV-snXXq4-vWLCpe-ehcLpc-2hHPQCa-2jCQdZL-VKRSby-MH1Ww1-Lp5GCw-zBeX2z-eXqrm2-MAGEn5-4UzU68-MJ1wwc-2grZm46-GtWSQH-9TEYPk-Kb44D7-4944rD-krmH1u-77AtmT-91iggW-9TEYX6-2hQxmo-4HMCqH-9iD5nA-4ewvQy-2is4voQ-2isi31-2CCfj-aCc62m-2eyBLm6-9TEYL6-4UEuqY-4DnRvT-zPTyw-ehdE9S-EoFkjU-92Pyy4-6osfY"),
        },
        ImageMetadata {
            file_name: String::from("49652643563_4fe8cb5026_k"),
            dims: Vec2::new(2048., 857.),
            title: String::from("Roy's Amboy"),
            author: String::from("Mobilus In Mobili"),
            url: String::from("https://www.flickr.com/photos/mobili/49652643563/in/photolist-2iDCM4r-2eKfveb-2ja9GBM-n9QXeD-2hQwC7-ehdUac-eaRDGg-9GrMpx-dq6mry-snYqna-9GuEky-9GrMFi-KZZB9i-9GrLZP-9GuEbG-snRvh5-9GrMxr-foa1ss-wNqWq-oBcLqm-9GrKJt-2hL9re-2hQCt5-dX3iBF-5hiH6Y-zfbLDn-4DnSnH-57W1yg-2gFTXsW-4N2UK1-TVxfEX-zc32WN-4DnV86-4DnUig-2h8v8vR-4Ds9bq-4zs1Z-5PQ46P-7g2qQr-bcqhnZ-t4UFe6-4DnRdD-TQVghM-iE3rM-5hiWNm-ehcaCK-4Qnxoj-25uWK-9JPUar-4UAeJD"),
        },
        ImageMetadata {
            file_name: String::from("49871620191_aa871df30b_k"),
            dims: Vec2::new(2048., 1366.),
            title: String::from("Chester Cathedral Choir 2"),
            author: String::from("Michael D Beckwith"),
            url: String::from("https://www.flickr.com/photos/185017394@N08/49871620191/in/photolist-2iYZ6bx-EUK7yN-wLs6To-2jvfYJC-P6Ph4Y-2hphkot-4XuM4d-25Gwrqs-2h6qgPd-5cSrV3-28J4Y9f-FhSsmU-G7t7yH-2ia8QyP-FNcYG7-dCXP13-WqjFM3-EpgUif-EFoG97-2hJTb8A-Fyh9rV-EJGSYS-G7t8ea-w4Fakr-GE6mMR-Fi49Gc-GdjqQt-FhSsaw-7JMfSe-V4YTLV-29ARvfd-EhVeLR-Fm89DK-2gVCfRS-2iUFHvZ-6kodM1-EpgVBY-FNcXhJ-29xemj7-EP65j1-DTFdLW-G7t4kZ-FhSs75-msFxrZ-msFLNM-msF698-54sEE9-msHq2o-2kuaPvk-r843zK"),
        },
        ImageMetadata {
            file_name: String::from("50259586112_eed250fbd4_k"),
            dims: Vec2::new(2048., 1365.),
            title: String::from("Brooklyn Bridge"),
            author: String::from("bryansjs"),
            url: String::from("https://www.flickr.com/photos/bryansjs/50259586112/in/photolist-2jzgvW9-GaSoiQ-FhTSPD-fjBZrF-ztvjYT-znj7Mf-G7t9dp-2cSHSDH-V5nH4-zunufe-zsbFZ8-7CjqHd-FhHe7f-p37K5b-p17RMA-a7heg3-DNBcQL-DPaFBc-yMGKsQ-o9Prpk-PoDtK1-G7iS2K-e6V8jC-HWiHTf-EFoFc7-e6V8Vw-EJeEtY-2jzgwjo-e6V8yQ-4tnDGE-GaSoEw-FN3NY7-ehkdE1-p37zfS-EcQWgP-q7Ux6P-DP2EDg-oKEhoZ-a7cVMX-p2T8cp-FhTUEn-FhTVjD-EpgWd7-ehdAoh-GdacY4-z6Lgze-FN3PK7-yMEvAL-5hDx5k-cfCKbJ"),
        },
        ImageMetadata {
            file_name: String::from("5075142940_afd0bdc5b1_o"),
            dims: Vec2::new(1800., 1232.),
            title: String::from("BMW Welt, Munich"),
            author: String::from("O Palsson"),
            url: String::from("https://www.flickr.com/photos/opalsson/5075142940/in/photolist-8Jtrbu-dQsgxy-HrN1Pt-yA8sg-p17Luw-oKDPT6-8nXkKU-s5tbSa-zHEwaL-oKEjCj-a7eAKe-zs92Gm-Ej6tgw-DNWWhv-7rQRtn-6cgbDh-2iAiCva-2jzgoQL-EcESXp-dZ7bjU-ouphb2-DNT5Gp-DNT5qc-2kfiaTk-dY2NgF-p37G65-DNT5aH-2cNW7MU-e6V7oq-oKEi85-5ejecv-oKDXoN-3iVUo9-iiGDwj-cJn47Y-6MPYpA-2kfoo8R-oKEpXN-8apoMQ-zvyov4-5eoADC-3pAJ4s-DNxpbj-3huoa2-p37Kr3-zw1u84-e6V7FS-5ejd6r-2jDG7c7-5ejdjR")
        },
    ];

    let mut rng = setup_random_seed();

    let i = if let Some(ii) = image_index {
        ii
    } else {
        get_usize(&mut rng, images.len())
    };

    return images[i].clone();
}
