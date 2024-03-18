// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/TheaLatest.sol";
import "../src/MMR.sol";
import "../src/Ds.sol";
import "../src/PolkadexTypes.sol";

contract TheaLatestTest is Test, Ds {
    TheaLatest thea;

    function setUp() public {
        uint256[2] memory vrf_public_key = [uint256(20149468923017862635785269351026469201343513335253737999994330121872194856517), uint256(45558802482409728232371975206855032011893935284936184167394243449917294149765)];
        address relayer = address(0x4B257d164c8F94c042AC3a64855618C863b8dBEe);
        //thea = new TheaLatest(vrf_public_key, relayer);
        thea = new TheaLatest();
        console.log("Setup is successful");
        // Insert validator into storage
        for (uint i = 0; i < 200; i++) {
            // Convert ecdsa public to to AccountId
            bytes memory ecdsa_pub_key = hex"020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1";
            bytes memory hash = abi.encodePacked(keccak256(ecdsa_pub_key));
            // Generate address using last 20 bytes of hash
            address addr;
            assembly {
                addr := mload(add(hash, 20))
            }
            //thea.addValidator(0, addr);
        }
        address[] memory validators = new address[](200);
        validators[0] = address(bytes20(hex"3e28864941dd72e5e5035fca6a621abfae9f7d1a"));
        validators[1] = address(bytes20(hex"cef9b9fdbc75883aae4465b27c43c1293338d345"));
        validators[2] = address(bytes20(hex"62062b757aad0ef4335d900fb62e01d9644e54d7"));
        validators[3] = address(bytes20(hex"84509b4abd358c307e33b543fe70965876fc2343"));
        validators[4] = address(bytes20(hex"b261350686b4789b5f4f9a1de0cd6b6194c0b0f8"));
        validators[5] = address(bytes20(hex"f2f249889e499fd218047feafbca4fbb980a7820"));
        validators[6] = address(bytes20(hex"e4ef4c169063332f9852eaf66ca85d724e50ed47"));
        validators[7] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[8] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[9] = address(bytes20(hex"67134fbf3aef9cb7f10f17038e9dc164efbd90f2"));
        validators[10] = address(bytes20(hex"d8e2f7378b7ef378b1c61e6a0d5817b59434054a"));
        validators[11] = address(bytes20(hex"33bef2161cd066474e3c76ff321bd963d848abdc"));
        validators[12] = address(bytes20(hex"22d160e41531e3b96ea37535b9ba68cf22244ceb"));
        validators[13] = address(bytes20(hex"0ca6eef238645aaac33c68998d4499eae1caaa7e"));
        validators[14] = address(bytes20(hex"9d82626da4c6e264460f204c715a6febe3ba40b5"));
        validators[15] = address(bytes20(hex"302a6f0b400822f5befc3477088638e5f27c256d"));
        validators[16] = address(bytes20(hex"d567f2048421cf2af1d4d0b1fd8ccaacb317f7dd"));
        validators[17] = address(bytes20(hex"e074769fdd57db4c62fffb5b11a28f0a4a9fb9f1"));
        validators[18] = address(bytes20(hex"7b8b53386ec59797419937d12d6a6216bffcd928"));
        validators[19] = address(bytes20(hex"8668ed2f3332906dd555d23746fc86d676820849"));
        validators[20] = address(bytes20(hex"5864e925749534788482855c4efd474804b953ef"));
        validators[21] = address(bytes20(hex"ecdd565d8ec6037eacd16d6135a7664eb507672d"));
        validators[22] = address(bytes20(hex"fba9e298db68875ebe6bdc9f66c1add2915552aa"));
        validators[23] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[24] = address(bytes20(hex"83c7ebb1ab1eb09105c4b200075dc6d0f415eff5"));
        validators[25] = address(bytes20(hex"4681245033e38a87f388f4b7215ce30aa90a90d7"));
        validators[26] = address(bytes20(hex"5683527ad9d875cd538bdf0f433c39c1dba9b3f4"));
        validators[27] = address(bytes20(hex"b3e21667eca17f85d7476df2fbe5694ea44aeeea"));
        validators[28] = address(bytes20(hex"0869e02653d3b75c665af63ac9628a11eed71e2e"));
        validators[29] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[30] = address(bytes20(hex"618637c88d516e5892096a28de05c5b46dc1de6c"));
        validators[31] = address(bytes20(hex"de15968f69fd143a9ba801f95fdee7fdbec31d2d"));
        validators[32] = address(bytes20(hex"a0265100d21c5e82855364fb79b20d7bda61a63f"));
        validators[33] = address(bytes20(hex"ce2ed054271a0fd2373ea0140e47e6b3266ad97f"));
        validators[34] = address(bytes20(hex"bbb0e4f17348fd4bec0a5bcfe9b951a64d791e24"));
        validators[35] = address(bytes20(hex"60f5b4d7c60c4265269989507e3bcd689e82e18a"));
        validators[36] = address(bytes20(hex"aba1cccc3ae6b1111e2dae287f1c50812fa29f8a"));
        validators[37] = address(bytes20(hex"4dad2bcc0f8599c589c2bf965219c06cb5a5688e"));
        validators[38] = address(bytes20(hex"c3d1624d348bee493b11c759aea3cf122aa4eb4a"));
        validators[39] = address(bytes20(hex"4c054f63b3cd78601a08a1e5799e27c25eca093f"));
        validators[40] = address(bytes20(hex"f60b0d6f510a5c4c5c09e05e2d492aaa26f65661"));
        validators[41] = address(bytes20(hex"83a58c4001c37c6e5a84d888498f46b54fc40f9e"));
        validators[42] = address(bytes20(hex"0ba7873fd24a7b481784d897be66641dd0e4c524"));
        validators[43] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[44] = address(bytes20(hex"58ea0a293dbf929fc4861937c34ca721c2b529c5"));
        validators[45] = address(bytes20(hex"1c97b964a7f3aa33d9f440bcf53212b7d353bb93"));
        validators[46] = address(bytes20(hex"6700fb158f62295ad36eb54a21cc8f04803f4ef9"));
        validators[47] = address(bytes20(hex"475a03565abd0679eaa1871cb5ee53115174d21b"));
        validators[48] = address(bytes20(hex"338d739d04028a8584b4a9a02b3f16a802ae0c94"));
        validators[49] = address(bytes20(hex"9610c59829c27e6262f248e0ce9c24abd23a9a4b"));
        validators[50] = address(bytes20(hex"3bc0e410418078d74660c9a1790c32e25f6d2e97"));
        validators[51] = address(bytes20(hex"fa7655f839ab4650ca1752deee8b464d3ca9c8af"));
        validators[52] = address(bytes20(hex"16d2dfa5789c8b6c6f9e7729fab13efa147c60e7"));
        validators[53] = address(bytes20(hex"792581e58713ceb7d7f9b394ad9bf0214cfff650"));
        validators[54] = address(bytes20(hex"5f6d8b092cec36895400c0b6eeef9391f26de33d"));
        validators[55] = address(bytes20(hex"4b012ed5b870bdd3b149bfd566dc0217fbba2c41"));
        validators[56] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[57] = address(bytes20(hex"c2959156be9237e51706f7d2731a21f2d19fbbda"));
        validators[58] = address(bytes20(hex"37caf8ca066f2ff2e171a4ffc2a05ef07b039834"));
        validators[59] = address(bytes20(hex"d8f8ada8f902244c8e529cc5e7170ea58abd1cf1"));
        validators[60] = address(bytes20(hex"184901437464d5ddcf8a8a82cddf5391754477ff"));
        validators[61] = address(bytes20(hex"cd1ddbb320598c766e40d979d03953049980b320"));
        validators[62] = address(bytes20(hex"1122f9a9376f445388a0cee2ba8c6447d0b6eb57"));
        validators[63] = address(bytes20(hex"50d6aab9bda24f7a4508618d4a6cf31282c3f411"));
        validators[64] = address(bytes20(hex"fafacd920ab49058a26755f099813c2250a2476f"));
        validators[65] = address(bytes20(hex"7432514ada35e000c00aed34eb12400c3ecc4fcf"));
        validators[66] = address(bytes20(hex"b32d61528ef1b477e6f9b157babcb3fd714f61fe"));
        validators[67] = address(bytes20(hex"4dddd0e57e555ba9b0576d73b70f62baa670dfd8"));
        validators[68] = address(bytes20(hex"d4e73e3817f0cc4c2108c8e8559ee848f5524312"));
        validators[69] = address(bytes20(hex"f580e6d8df28e84407cf85a053fca040e01899d6"));
        validators[70] = address(bytes20(hex"1c85ca149c651086e031f70c6cb5797b21ad398f"));
        validators[71] = address(bytes20(hex"7960bef7dadff1ed176c4d6d592be4195173aa0f"));
        validators[72] = address(bytes20(hex"20c979520852703ddf126cd96985f2120dc58225"));
        validators[73] = address(bytes20(hex"24243aada6831b9f0ba62fe5fe80ee3620efaec3"));
        validators[74] = address(bytes20(hex"9715d157272d58672736a02fe66070256c235d56"));
        validators[75] = address(bytes20(hex"fe4fbd1b849f45d747483303a449ef0678c4c967"));
        validators[76] = address(bytes20(hex"f60ceedab562659fea9927f26b0a8dd9f53d7023"));
        validators[77] = address(bytes20(hex"ffe9479ac94475d2fc91d413d45d4d4b316a9083"));
        validators[78] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[79] = address(bytes20(hex"cfa911d60479b9a5f133600f5ffa38602139af79"));
        validators[80] = address(bytes20(hex"494f5247e7bb530efad3f5d621b52d0799e19d09"));
        validators[81] = address(bytes20(hex"c3e1b2ca8a95c0c5a131383b97585f57cdabe14e"));
        validators[82] = address(bytes20(hex"9bc98b52759985f9f178f0ebad69041d09011b28"));
        validators[83] = address(bytes20(hex"346f60c0e430ec972acba8d3fba54c3781b42eb9"));
        validators[84] = address(bytes20(hex"711683023ad342e4539610350046aaa82d091454"));
        validators[85] = address(bytes20(hex"43bc0cbe0e0eeb436858604bc05a745928591dd5"));
        validators[86] = address(bytes20(hex"9e4b5e159f0909ba314173a45aef14e65049af3c"));
        validators[87] = address(bytes20(hex"3fd90defc2ced35dfb00837a3d7c3e6f37faf416"));
        validators[88] = address(bytes20(hex"8bf335d10d942ee2560186b528fe9e1d1e96d642"));
        validators[89] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[90] = address(bytes20(hex"7d470284d7658b364a0cab607cb9bf11844585d3"));
        validators[91] = address(bytes20(hex"2ac4bd7bae8f4cae3d050a4e4e3b78c1b66ca74e"));
        validators[92] = address(bytes20(hex"0f1d27fa7b36c17d38c60e55e2c21b9a5ffbcfc2"));
        validators[93] = address(bytes20(hex"c17dcf319dc5db37a66dcc5da1a91fb67abbfc06"));
        validators[94] = address(bytes20(hex"ab45e2409e39aa2021b407bc6c1e0e4f42b7e4a1"));
        validators[95] = address(bytes20(hex"257526c8a45d1aa28a828a1b4f89bf8eb612bdac"));
        validators[96] = address(bytes20(hex"2237a3e4a33ab1bf9e2250516e7652e9c1a91bea"));
        validators[97] = address(bytes20(hex"c399d896a2598340f508b2d7544e4ea115ab9150"));
        validators[98] = address(bytes20(hex"5495f9d8b2777423209d508ddb6c054bdd8eace8"));
        validators[99] = address(bytes20(hex"1b36187272ef0a5681b9843931674613896f456a"));
        validators[100] = address(bytes20(hex"8b2a7af5c25b0f12c64846374245cda994130fc3"));
        validators[101] = address(bytes20(hex"a190714e78bc7db1bcf18ea31c7ee5d2adb8fc0d"));
        validators[102] = address(bytes20(hex"3ce877feaadbfc562d32b7729514ce19a23d7ebd"));
        validators[103] = address(bytes20(hex"195704c9bbbe1e1b2497a2b0c389d38535adf189"));
        validators[104] = address(bytes20(hex"70e4cd871177cf692e3ff1efa716a73dab0ca0c7"));
        validators[105] = address(bytes20(hex"fe1d07ac387791cb6baa14d67ee2f065d98d007a"));
        validators[106] = address(bytes20(hex"3cbd5ffb828a1d97f8a4640deaec9b96ef817908"));
        validators[107] = address(bytes20(hex"c7379c6bebf83ec158278c50976758c640a0a7ab"));
        validators[108] = address(bytes20(hex"e665db7b946ea51a19658de0ae2e36e85ae5e8a1"));
        validators[109] = address(bytes20(hex"10ea03937f76d5bb331e377f7bbd9cfe3506d41f"));
        validators[110] = address(bytes20(hex"dbc3e0e36322118040d6b312d81ecc2c4dbe52d5"));
        validators[111] = address(bytes20(hex"ab01036a3d6a2ea423b1459affffcdb1b24d1587"));
        validators[112] = address(bytes20(hex"91db1e19e05767a2c98e1a00fe5d42e02735ca55"));
        validators[113] = address(bytes20(hex"498fd23c09977331f67274f6339692b5a3718785"));
        validators[114] = address(bytes20(hex"9d7e815561d368cb1d12c0bc9db71c27137a70ca"));
        validators[115] = address(bytes20(hex"c33013eb252b96f768f48f20fa7b6b93bc9239a3"));
        validators[116] = address(bytes20(hex"eee8deeb1fa1d850c6875017bd82644794369f0f"));
        validators[117] = address(bytes20(hex"971291dd4c3843cfcfa164981e54d385c2857f37"));
        validators[118] = address(bytes20(hex"f98e075f4a6ab5caede827d6b961f65ee757f52a"));
        validators[119] = address(bytes20(hex"3459515601bbe050adfbda13d141bb07d78aa137"));
        validators[120] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[121] = address(bytes20(hex"471f7dc244649c9d59044e47427e6f8a8eee1e67"));
        validators[122] = address(bytes20(hex"4bbbcfd29cdb50b819fd5c2940fbd937dd84f23d"));
        validators[123] = address(bytes20(hex"bdd47a2924c6f574c89f32a12ad9676dc692698f"));
        validators[124] = address(bytes20(hex"aae55bc882cb6785cc2e7f348ee29b18fd71dc4a"));
        validators[125] = address(bytes20(hex"07ddf8cad5ecff60d9c1a3d8e8679a338eaa2f8f"));
        validators[126] = address(bytes20(hex"d3f5097d5e1518f4f2b53cd2a70896f7695a5cd4"));
        validators[127] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[128] = address(bytes20(hex"a1afe73698831011d1c0944c29c2103a09d07310"));
        validators[129] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[130] = address(bytes20(hex"366b22f4813dc506d06d493841a87aec3f70d6bc"));
        validators[131] = address(bytes20(hex"a7d2c9cd15aed63821607f612558112f017cd915"));
        validators[132] = address(bytes20(hex"e4a11ac88b3812718a83a0f31ecb3ce1cb676602"));
        validators[133] = address(bytes20(hex"40957308dbfba244f1759c26dfd00ffa91dbb065"));
        validators[134] = address(bytes20(hex"bd09ef5116440726d0f018c7ecff1a99a55fd47b"));
        validators[135] = address(bytes20(hex"a6459631410ba59e3c5c1a803e481e7a18c380b3"));
        validators[136] = address(bytes20(hex"57e55d31effa86fd7d5557c92d40be8296fda3ff"));
        validators[137] = address(bytes20(hex"931d0f8918c9cc1f3864c14b9a0b39a02b42a2fa"));
        validators[138] = address(bytes20(hex"7a2524bc785ce1cc360cfcb4b4036746b425af0e"));
        validators[139] = address(bytes20(hex"493c5848519ae46566753038763db72792aef39c"));
        validators[140] = address(bytes20(hex"bda15f1de644f113abe275f149a3c1ad7ee1d9ae"));
        validators[141] = address(bytes20(hex"ac3acbbb926b2656a9f3e69beda653b2c0f34642"));
        validators[142] = address(bytes20(hex"c149212748d73237b4195a3ddc85c75eb685bcdb"));
        validators[143] = address(bytes20(hex"e601a2284dd3092df4f9da0c1bf3f05724ddf66f"));
        validators[144] = address(bytes20(hex"1effa5937d6a025d962e166a275da3cbbc2c0626"));
        validators[145] = address(bytes20(hex"8b5f2f65979c0febbbcb5a6ebfd12ac4233eb2a0"));
        validators[146] = address(bytes20(hex"96d9eaeb4b97e9440410eae5d02b6267f97ca5d8"));
        validators[147] = address(bytes20(hex"fe35168ca6cf0d116c230de03175eddff5388038"));
        validators[148] = address(bytes20(hex"8ce93a05771f1e08acf21dd384eaf322800735c4"));
        validators[149] = address(bytes20(hex"505bb0a155cc4aca0d9e68a568c2b1a1f2118e03"));
        validators[150] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[151] = address(bytes20(hex"2f5c4e86463892c9c0f48f4b8d729f4324b10a33"));
        validators[152] = address(bytes20(hex"3e0c92321eb63adb323dfc94a083f2e2e26ac623"));
        validators[153] = address(bytes20(hex"c1e0c2b8ed0448fdc7ec22716771e2673084d734"));
        validators[154] = address(bytes20(hex"5cb0ae4ebdd896fa230107256130257ab402f028"));
        validators[155] = address(bytes20(hex"92a5ed5d0e2dbe6684501aafc476f7b6b2007758"));
        validators[156] = address(bytes20(hex"1791dac92126f2b9429cc31153b1cd266abfce8b"));
        validators[157] = address(bytes20(hex"1a13481f193dca41849dd9a0de8ceb245585e33c"));
        validators[158] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[159] = address(bytes20(hex"0227e6711be8863bbaa4c296614e6249a5902906"));
        validators[160] = address(bytes20(hex"2c7044103e89137ef4dbde3d3e68807401f526dd"));
        validators[161] = address(bytes20(hex"fae6daefa32e9112c8ba88058b4e49ad22451e6c"));
        validators[162] = address(bytes20(hex"fea87e580f531192c0d6b3694843e04d6dae8fdf"));
        validators[163] = address(bytes20(hex"969384e44b3995c5b7ecff0eec97d232e2769cf4"));
        validators[164] = address(bytes20(hex"b801bd717c438e8e2f108a6a0082731f0264dcdb"));
        validators[165] = address(bytes20(hex"1d721af04c45637d83517c202ad9f97244f4e270"));
        validators[166] = address(bytes20(hex"99183b7557684e378f099207397a9a2a73220143"));
        validators[167] = address(bytes20(hex"c4fe856708e718cccef319896a60b0815edbcddc"));
        validators[168] = address(bytes20(hex"042d89e80951b9f9e43dac9dd95946869be15062"));
        validators[169] = address(bytes20(hex"5ed454628f0e7e66cd90e36935bcbcefae9ffcab"));
        validators[170] = address(bytes20(hex"8c325d6a726b782cc8942bff380a1abe421f7606"));
        validators[171] = address(bytes20(hex"c49809a1cd637baf789c838e08bbd65b178dbc73"));
        validators[172] = address(bytes20(hex"139acab1445f3b0bb5bfe09370fc42e164cb54bf"));
        validators[173] = address(bytes20(hex"a595ae85438cb687d4ac3a867b7f35b923cd9e18"));
        validators[174] = address(bytes20(hex"f1d8dd878c0bdafafe4aaa4930d35f15815f9ab9"));
        validators[175] = address(bytes20(hex"0000000000000000000000000000000000000000"));
        validators[176] = address(bytes20(hex"9455a39230d86e2be1fe543b0943310281e3bde3"));
        validators[177] = address(bytes20(hex"d0489a1ce80e609bc42ee7ae340029df2e8dcce9"));
        validators[178] = address(bytes20(hex"a14cc314320e1a8835cb3a116a33fdd5e8e06245"));
        validators[179] = address(bytes20(hex"a4fa08e34070a84daf24bf03fdc8a3ce02049907"));
        validators[180] = address(bytes20(hex"0c2c41601995486107b45ac50251deb284ba72e2"));
        validators[181] = address(bytes20(hex"510b5708ee6bd5993251030e8e4b080fbd28bdde"));
        validators[182] = address(bytes20(hex"0dbba2ed9219bc8f0d5cc190d62dfe5d5452af28"));
        validators[183] = address(bytes20(hex"5f60310109b0f51924c4d28dde91ea34ac1eacc5"));
        validators[184] = address(bytes20(hex"038d1e5020bd7de760f34f12c4c2fd90b7682dc8"));
        validators[185] = address(bytes20(hex"d31bde44f4fd1a7211bc4f1a7d5edeb7b6a9a44e"));
        validators[186] = address(bytes20(hex"db41c1f14bec01960fc599197f8ed6418cb33538"));
        validators[187] = address(bytes20(hex"fdca61b7c2853415a3291777b2d893043f9d28c5"));
        validators[188] = address(bytes20(hex"278cc992c645713f90adcd64e83d694a70e0b99d"));
        validators[189] = address(bytes20(hex"fef384c492afd2558505d2fc6c085f85fc36763a"));
        validators[190] = address(bytes20(hex"43eec8d56c1f55ca993964ba879cfa17730375a0"));
        validators[191] = address(bytes20(hex"73f15bb4e7db8323f4e7ab0fa12d9874ec32a53a"));
        validators[192] = address(bytes20(hex"6cf7337b26e96fc6ff12fc2dbb53f73390807177"));
        validators[193] = address(bytes20(hex"8f13d8b8a986938733e2b65210274f2c65ad1daa"));
        validators[194] = address(bytes20(hex"573089e91945b30f87f49e517b313d651fbda2d4"));
        validators[195] = address(bytes20(hex"10bea2175638bb6993136d6e4d8a77184d5d0e2b"));
        validators[196] = address(bytes20(hex"e9046ec409aea15486407eba21cc7d96cc180c37"));
        validators[197] = address(bytes20(hex"71af22e41adbab9ba81abe5f31be65b188b086ec"));
        validators[198] = address(bytes20(hex"71164175d6ab76b285085ed62389b3f4c022e240"));
        validators[199] = address(bytes20(hex"d634612cb55ec842f8db63db6fe903047e052bd0"));
        uint64[] memory validatorsIndexForVerification = new uint64[](1);
        validatorsIndexForVerification[0] = 0;
        thea.initialize(address(0), 243, validators, 10, validatorsIndexForVerification, 0, 0);
    }

//    function testDecodeWithdrawal() public {
//        bytes memory data = hex"0c280202020202020202020202000000000000000000000000000000030000000000000000000000000000005004040404040404040404040404040404040404040004052802020202020202020202020000000000000000000000000000000300000000000000000000000000000050040404040404040404040404040404040404040400040528020202020202020202020200000000000000000000000000000003000000000000000000000000000000500404040404040404040404040404040404040404000405";
//        PolkadexTypes.decodeWithdrawals(data);
//    }
//
//    function testDecodeRawWithdrawal() public {
//        bytes memory data = hex"0428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
//        PolkadexTypes.decodeRawWithdrawals(data);
//    }

//    function testDecodeMessage() public {
//        bytes memory data = hex"44000000000000000100000000000000020000000000000000020d010428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
//        PolkadexTypes.decodePayload(data);
//    }
//
//    //44000000000000000100000000000000020000000000000000020d010428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000
//
//    function testSendMessage() public {
//        bytes memory message = hex"44000000000000000100000000000000020000000000000000020d010428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
//        bytes memory signature = hex"8591e3639711056d42221705753f52fbe65e7ab8d5bdcc8533fc74f74fc77edf5f49a760835af86adaa4c5143ca3ff20cc84cd8b3f26068f1caad63ecd065f621c";
//        thea.sendMessage(message, signature);
//    }
//
//    function testSendMessageWithVrfProof() public {
//        thea.changeMode(Mode.Validators);
//        bytes memory message = hex"0b000000000000000100000000000000020000000000000000020d010428b36010eb285c154a8cd60c0000000000000000000000000000000a000000000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
//        bytes memory signature = hex"0ec8b09f096d1d7a5e9efc57cd1d6db15ddfbb0025e4c43cb83873c79fabf37f4f46db269760211b14bbb707a153806d7d57162f5aba28578de1f7b85a9032b201";
//        bytes memory vrf_proof = hex"0208330248d8de6f1b48cc0d7fa907e7c6507d313754706646da1af62b52524e44253bdc2dd2f32ff3635b2aa989355d10ba74126ebbdae84ede03272543c8df26b479cc476565f56ff1095b34e613a93c";
//        bytes[] memory signatures = new bytes[](133);
//        for (uint i = 0; i < 133; i++) {
//            signatures[i] = signature;
//        }
//        thea.validatorValidation(message, signatures, 0);
//    }

//    function testDecodePayload() public {
//        bytes memory data = hex"bc01000000000000010000000000000002020d010428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b00e1f505000000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
//        PolkadexTypes.decodePayload(data);
//    }

//    function testSendMessage() public {
//        bytes memory message = hex"1d00000000000000010000000000000002020d010428b36010eb285c154a8cd6af098e0cc9d1d058b55527316ba8788b0010a5d4e80000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
//        bytes memory signature = hex"3af2bd40acec27bb32b2583c15dc3a969a256e64cc95f0c29da36820aeaf4ee7291fcde0c7b467e998c15d3017567b74938fd9be07f40e7f072a2d8f4890bc6b00";
//        bytes[] memory signatures = new bytes[](1);
//        signatures[0] = signature;
//        bytes[] memory validators = new bytes[] (1);
//        validators[0] = hex"02ef1e5505d809b1d3624c2fc8ab6346e370ab13e9b8675cbdd5481ed3be289abf";
//        uint64[] memory validatorsIndexForVerification = new uint64[](1);
//        validatorsIndexForVerification[0] = 0;
//        thea.initialize(address(0), 1, validators, 1, validatorsIndexForVerification);
//        thea.sendMessage(message, signatures);
//    }

    function testSendMessage() public {
        bytes memory message = hex"5d7c5f00000000001f00000000000000ff020d010428d601c93fbebcfb8b57276f0000000000000000000000000000000000c16ff28623000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
        bytes[] memory signatures = new bytes[](10);
        signatures[0] = hex"e3fb429a540663fdd7539b1d0f37acf0eacc3b6a87a18dc044ff29eb84f1c8be0dbf87873afceaafc43070b317e76fe30ceb71c7b0e858dc6d5d42cb0436cc9900";
        signatures[1] = hex"46d735e5a9bf73d6067caa7871a1669ac384e58bf924da0d04253ec4efcdb0563e705dd3d8a4c98ee76474878e927574a9f88b10a618a392bb8d30556fecc70801";
        signatures[2] = hex"681528d6560c58b53ee5735e3179156c998d649e4e108e5acf24279181d11c0d397724da3a0780bcc0aeea51fb65ffd468f17045c7b79444d69e9273431d1f1f00";
        signatures[3] = hex"07b0465e958f21d707f4bbbad0c3656452444425697b507e65fe1ee98327e54548483ce322821a47ed94008af28f55bc514962763d646d539f0d4762f820e92100";
        signatures[4] = hex"8ff316bfd7b42973c10a26cba561439a53cc0ff68553f55926e75f9514effd472a32541c04a9d35f175f61b8adc6bd1ea59de54506449a865528625333e32dd000";
        signatures[5] = hex"1950b432aa562ec04a0210f45beedb0b3a36c6b0f66627510522a685761482ef6e186a623e705ab6fa1cce9e05d6610a729bc8fdf8d861647579d90720ce33a501";
        signatures[6] = hex"d5fb31170a89488d5f4f546bd570ea32700402f38b5767ded4612c2557d84feb312d07e1d913bb21e26b7a95043c215a5a5501c43df5bea759801febbfde447e00";
        signatures[7] = hex"8291318bb838033455998a20e66f3ff9741210537c69a1b5d7ffff5e03bd87af16c4883e7a9d486a09532e75d6783332d49bac701be44d9746676e8a9f02e08d00";
        signatures[8] = hex"9df322aafa37bbccfd4f39b9a713398c13b4d05be44f963c758ce6627621e0b426921a8750679b4879bcdbd09345459d95b8049ec629793fde9b1da5c51d869701";
        signatures[9] = hex"6b93efaf15bfd9c208c659b857565b46bf3f68cc60b236d3bd7e14893010c64e2c495ddafa5e32435a39becd87728a5bbe2036d64297e91b6a83ea434399aaaf00";
        // Fix follwing line
        uint64[134] memory validatorsIndexForVerificationNew = [uint64(1), 3, 4, 5, 6, 11, 13, 15, 18, 21, 22, 25, 27, 28, 30, 31, 32, 33, 34, 36, 37, 38, 40, 41, 42, 44, 46, 49, 50, 52, 53, 54, 55, 57, 58, 60, 62, 63, 64, 67, 69, 70, 71, 72, 73, 75, 76, 77, 80, 81, 82, 85, 86, 87, 88, 90, 91, 92, 93, 94, 95, 96, 97, 98, 101, 104, 105, 108, 111, 113, 114, 116, 117, 119, 121, 122, 123, 124, 125, 126, 130, 131, 133, 134, 135, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 149, 151, 152, 153, 154, 155, 159, 160, 161, 162, 163, 164, 165, 166, 168, 169, 170, 171, 172, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 188, 190, 192, 193, 194, 195, 196, 197, 198, 199];
        uint64[] memory validatorsIndexForVerificationNewDynamic = new uint64[](134);
        for (uint i = 0; i < 134; i++) {
            validatorsIndexForVerificationNewDynamic[i] = validatorsIndexForVerificationNew[i];
        }
        thea.sendMessage(message, signatures, validatorsIndexForVerificationNewDynamic);
    }

    function testAbcValidatorValidation() public {
        bytes memory message = hex"be975e00000000000b00000000000000ff020d010428d297a4221c3273a076266f0000000000000000000000000000000010a5d4e80000000000000000000000504b257d164c8f94c042ac3a64855618c863b8dbee0000";
        bytes memory signature = hex"d54d91ec34970020d7b57158a6c191a9096ac446552c860659b5f49eee81f5426be17a1157c027124b21c460319ae301e464ebcd8be4d70adaa21943ac688fec01";
        address validator = address(bytes20(hex"2a7cc7c16d761a7ec5a14a53343c0dcbbab0db"));
        thea.abcValidatorValidation(message, signature, validator);
    }
}
