/// Message element type define
/// Address needs to be 
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
pub struct InkAddressData {
    pub ink_address: Option<[u8; 32]>,
    pub general_address: Option<ink_prelude::string::String>,
    pub address_type: u8,
}

impl ::scale_info::TypeInfo for InkAddressData {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("InkAddressData", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<Option<[u8; 32]>>().name("ink_address").type_name("Option<[u8; 32]>"))
                        .field(|f| f.ty::<Option<ink_prelude::string::String>>().name("general_address").type_name("Option<ink_prelude::string::String>"))
                        .field(|f| f.ty::<u8>().name("address_type").type_name("u8"))
                    )
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive())]
pub enum MsgDetail{
    InkString(ink_prelude::string::String),
    InkU8(u8),
    InkU16(u16),
    InkU32(u32),
    InkU64(u64),
    InkU128(u128),
    InkI8(i8),
    InkI16(i16),
    InkI32(i32),
    InkI64(i64),
    InkI128(i128),
    InkStringArray(ink_prelude::vec::Vec<ink_prelude::string::String>),
    InkU8Array(ink_prelude::vec::Vec<u8>),
    InkU16Array(ink_prelude::vec::Vec<u16>),
    InkU32Array(ink_prelude::vec::Vec<u32>),
    InkU64Array(ink_prelude::vec::Vec<u64>),
    InkU128Array(ink_prelude::vec::Vec<u128>),
    InkI8Array(ink_prelude::vec::Vec<i8>),
    InkI16Array(ink_prelude::vec::Vec<i16>),
    InkI32Array(ink_prelude::vec::Vec<i32>),
    InkI64Array(ink_prelude::vec::Vec<i64>),
    InkI128Array(ink_prelude::vec::Vec<i128>),
    InkAddress(InkAddressData),
    UserData(ink_prelude::vec::Vec<u8>),
}

impl ::scale_info::TypeInfo for MsgDetail {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("MsgDetail", module_path!()))
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant("InkString", |v| v.index(0).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::string::String>())))
                                .variant("InkU8", |v| v.index(1).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<u8>())))
                                .variant("InkU16", |v| v.index(2).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<u16>())))
                                .variant("InkU32", |v| v.index(3).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<u32>())))
                                .variant("InkU64", |v| v.index(4).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<u64>())))
                                .variant("InkU128", |v| v.index(5).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<u128>())))
                                .variant("InkI8", |v| v.index(6).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<i8>())))
                                .variant("InkI16", |v| v.index(7).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<i16>())))
                                .variant("InkI32", |v| v.index(8).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<i32>())))
                                .variant("InkI64", |v| v.index(9).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<i64>())))
                                .variant("InkI128", |v| v.index(10).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<i128>())))
                                .variant("InkStringArray", |v| v.index(11).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<ink_prelude::string::String>>())))
                                .variant("InkU8Array", |v| v.index(12).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<u8>>())))
                                .variant("InkU16Array", |v| v.index(13).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<u16>>())))
                                .variant("InkU32Array", |v| v.index(14).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<u32>>())))
                                .variant("InkU64Array", |v| v.index(15).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<u64>>())))
                                .variant("InkU128Array", |v| v.index(16).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<u128>>())))
                                .variant("InkI8Array", |v| v.index(17).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<i8>>())))
                                .variant("InkI16Array", |v| v.index(18).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<i16>>())))
                                .variant("InkI32Array", |v| v.index(19).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<i32>>())))
                                .variant("InkI64Array", |v| v.index(20).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<i64>>())))
                                .variant("InkI128Array", |v| v.index(21).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<i128>>())))
                                .variant("InkAddress", |v| v.index(22).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<InkAddressData>())))
                                .variant("UserData", |v| v.index(23).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<ink_prelude::vec::Vec<u8>>())))
                                //.variant("UserData", |v| v.index(22).fields(::scale_info::build::Fields::unnamed().field(|f| f.ty::<u8>()).field(|f| f.ty::<ink_prelude::vec::Vec<u8>>())))
                        )
    }
}

// pub trait MsgDetailTrait{
//     // fn get_type_value(&self) -> MsgDetail;
// }

/// enum variants are not `types`, we cannot generic more!
// impl MsgDetailTrait for MsgDetail::InkI128 {
    
// }

pub trait InMsgType {
    type MyType;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType>;
    fn create_message(msg_detail: Self::MyType) -> MsgDetail;
}

/// enum variants are not `types`, we cannot generic more!
/// impl for `ink_prelude::string::String`
impl InMsgType for ink_prelude::string::String{
    type MyType = ink_prelude::string::String;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkString(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkString(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<ink_prelude::string::String>{
    type MyType = ink_prelude::vec::Vec<ink_prelude::string::String>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkStringArray(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkStringArray(msg_detail)
    }
}

impl InMsgType for u8{
    type MyType = u8;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU8(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU8(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<u8>{
    type MyType = ink_prelude::vec::Vec<u8>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU8Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU8Array(msg_detail)
    }
}

impl InMsgType for u16{
    type MyType = u16;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU16(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU16(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<u16>{
    type MyType = ink_prelude::vec::Vec<u16>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU16Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU16Array(msg_detail)
    }
}

impl InMsgType for u32{
    type MyType = u32;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU32(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU32(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<u32>{
    type MyType = ink_prelude::vec::Vec<u32>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU32Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU32Array(msg_detail)
    }
}

impl InMsgType for u64{
    type MyType = u64;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU64(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU64(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<u64>{
    type MyType = ink_prelude::vec::Vec<u64>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU64Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU64Array(msg_detail)
    }
}

impl InMsgType for u128{
    type MyType = u128;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU128(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU128(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<u128>{
    type MyType = ink_prelude::vec::Vec<u128>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkU128Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkU128Array(msg_detail)
    }
}

impl InMsgType for i8{
    type MyType = i8;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI8(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI8(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<i8>{
    type MyType = ink_prelude::vec::Vec<i8>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI8Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI8Array(msg_detail)
    }
}

impl InMsgType for i16{
    type MyType = i16;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI16(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI16(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<i16>{
    type MyType = ink_prelude::vec::Vec<i16>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI16Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI16Array(msg_detail)
    }
}

impl InMsgType for i32{
    type MyType = i32;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI32(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI32(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<i32>{
    type MyType = ink_prelude::vec::Vec<i32>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI32Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI32Array(msg_detail)
    }
}

impl InMsgType for i64{
    type MyType = i64;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI64(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI64(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<i64>{
    type MyType = ink_prelude::vec::Vec<i64>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI64Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI64Array(msg_detail)
    }
}

impl InMsgType for i128{
    type MyType = i128;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI128(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI128(msg_detail)
    }
}

impl InMsgType for ink_prelude::vec::Vec<i128>{
    type MyType = ink_prelude::vec::Vec<i128>;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkI128Array(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkI128Array(msg_detail)
    }
}

impl InMsgType for InkAddressData {
    type MyType = InkAddressData;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
        if let MsgDetail::InkAddress(val) = type_value.clone() {
            Some(val)
        } else {
            None
        }
    }

    fn create_message(msg_detail: Self::MyType) -> MsgDetail {
        MsgDetail::InkAddress(msg_detail)
    }
}

/// for user data, see `UserMessage` in lib.rs for detail

// impl<T> MsgValueOut for T 
// where
//     T: InMsgType,
// {
//     type MyType = T;
//     fn get_value<MyType, ET: MsgDetailTrait>(type_value: & MsgDetail) -> Option<Self::MyType> {
//         if let ET(val) = type_value.clone() {
//             Some(val)
//         } else {
//             None
//         }
//     }
// }

/// Message Item, used for describing the information composed with a single element
/// @member `n`: item unique ID, which is used for user applications to communicate user-defined informations
/// @member `tv`: item type and the information data
#[derive(Debug, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive())]
pub struct MessageItem{
    pub n: ink_prelude::string::String,
    pub tv: MsgDetail,
}

impl PartialEq for MessageItem {
    fn eq(&self, other: &MessageItem) -> bool{
        return self.n == other.n;
    }
}

impl ::scale_info::TypeInfo for MessageItem {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("MessageItem", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<ink_prelude::string::String>().name("n").type_name("ink_prelude::string::String"))
                        .field(|f| f.ty::<MsgDetail>().name("tv").type_name("MsgDetail"))
                    )
    }
}

impl MessageItem {
    pub fn from(n: ink_prelude::string::String, tv: MsgDetail) -> Self {
        Self {
            n, 
            tv,
        }
    }

    pub fn in_to<T: scale::Decode + InMsgType>(&self) -> Option<T::MyType>{
        T::get_value(&self.tv)
    }
}

/// Message Payload
/// @member items: a vector of `MessageItem`
/// @member vecs: a vector of `MessageVec`
#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
// #[cfg_attr(feature = "std", derive())]
pub struct MessagePayload{
    pub items: Option<ink_prelude::vec::Vec<MessageItem>>,
}

impl ::scale_info::TypeInfo for MessagePayload {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("MessagePayload", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<Option<ink_prelude::vec::Vec<MessageItem>>>().name("items").type_name("Option<ink_prelude::vec::Vec<MessageItem>>"))
                    )
    }
}

impl MessagePayload{
    pub fn new() -> MessagePayload{
        MessagePayload {
            items: None,
        }
    }

    /// for `item`
    pub fn push_item(&mut self, n: ink_prelude::string::String, tv: MsgDetail) -> bool {
        let msg_item = MessageItem::from(n, tv);
        if let Some(item) = &mut self.items {
            if item.contains(&msg_item){
                return false;
            }

            item.push(msg_item);
            true
        } else{
            let mut item_vec = ink_prelude::vec::Vec::new();
            item_vec.push(msg_item);
            self.items = Some(item_vec);
            true
        }
    }

    pub fn get_item(&self, msg_n: ink_prelude::string::String) -> Option<&MessageItem>{
        if let Some(item) = &self.items {
            for it in item.iter() {
                if it.n == msg_n {
                    return Some(it);
                }
            }
        }

        None
    }

    pub fn to_bytes(&self) -> ink_prelude::vec::Vec<u8> {
        let mut pl_code: ink_prelude::vec::Vec<u8> = ink_prelude::vec::Vec::<u8>::new();
        scale::Encode::encode_to(self, &mut pl_code);
        pl_code
    }
}


#[cfg(test)]
mod test {
    
}