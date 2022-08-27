use crate::binds::MonoTableInfo;
use crate::Image;

pub struct MetadataTableInfo{
    pub table:*const MonoTableInfo,
    pub kind:MetadataTableKind,
}
pub type MetadataToken = u32;
#[repr(u32)] #[derive(PartialEq)]
pub enum MetadataTableKind{
    Module              =   crate::binds::MonoMetaTableEnum_MONO_TABLE_MODULE,
    TypeRef             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPEREF,
    TypeDef             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPEDEF,
    FieldPointer        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELD_POINTER,
    Field               =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELD,
    MethodPointer       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHOD_POINTER,
    Method              =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHOD,
    ParamPointer        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PARAM_POINTER,
    Param               =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PARAM,
    InerfaceImpl        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_INTERFACEIMPL,
    MemberRef           =   crate::binds::MonoMetaTableEnum_MONO_TABLE_MEMBERREF,
    Constant            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_CONSTANT,
    CustomAttribute     =   crate::binds::MonoMetaTableEnum_MONO_TABLE_CUSTOMATTRIBUTE,
    FieldMarshal        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDMARSHAL,
    DeclSceurity        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_DECLSECURITY,
    ClassLayout         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_CLASSLAYOUT,
    FieldLatout         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDLAYOUT,
    StandaloneSig       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_STANDALONESIG,
    EventMap            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENTMAP,
    EventPointer        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENT_POINTER,
    Event               =   crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENT,
    PropertyMap         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTYMAP,
    PropertyPointer     =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTY_POINTER,
    Property            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTY,
    MethodSemantics     =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODSEMANTICS,
    MethodImpl          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODIMPL,
    ModuleRef           =   crate::binds::MonoMetaTableEnum_MONO_TABLE_MODULEREF,
    TypeSpec            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPESPEC,
    ImplMap             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_IMPLMAP,
    FieldRVA            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDRVA,
    Unused6             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED6,
    Unused7             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED7,
    Assembly            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLY,
    AssemblyProcessor   =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYPROCESSOR,
    AssemblyOS          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYOS,
    AssmeblyRef         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREF,
    AssmeblyRefProcessor=   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFPROCESSOR,
    AssmeblyRefOS       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFOS,
    File                =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FILE,
    ExportedType        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_EXPORTEDTYPE,
    ManifestResource    =   crate::binds::MonoMetaTableEnum_MONO_TABLE_MANIFESTRESOURCE,
    NestedClass         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_NESTEDCLASS,
    GenericParam        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_GENERICPARAM,
    MethodSpec          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODSPEC,
    GenericParamConstraint= crate::binds::MonoMetaTableEnum_MONO_TABLE_GENERICPARAMCONSTRAINT,
    Unused8             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED8,
    Unused9             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED9,
    Unused10            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED10,
    Document            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_DOCUMENT,
    MethodBody          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODBODY,
    LocalScope          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALSCOPE,
    LocalVariable       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALVARIABLE,
    LocalConstant       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALCONSTANT,
    ImportScope         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_IMPORTSCOPE,
    MachineMethod       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_STATEMACHINEMETHOD,
}
impl MetadataTableInfo{
    ///*table* must be a valid [`MonoTableInfo`] pointer, and must match kind.
    pub unsafe fn from_ptr(table:*const MonoTableInfo,kind:MetadataTableKind)->MetadataTableInfo{
        return Self{table:table,kind:kind};
    }
    ///Get ammout of rows in a table.
    pub fn get_table_rows(&self)->i32{
        return unsafe{crate::binds::mono_table_info_get_rows(self.table)};
    }
    ///Gets the token at *column* in *row*
    pub fn decode_row_col(&self,row:i32,column:u32)->MetadataToken{
        return unsafe{crate::binds::mono_metadata_decode_row_col(self.table,row,column)};
    }
}
pub struct AssemblyMetadata{
    pub hash_alg:HashAlgorithm,
    pub major_version:u32,
    pub minor_version:u32,
    pub build_number:u32,
    pub rev_number:u32,
    pub flags:AssemblyFlags,
    pub public_key:u32,
    name:u32,
    pub culture:u32,
    img:Image,
}
impl AssemblyMetadata{
    fn from_meta_table(table:&MetadataTableInfo,img:&Image)->AssemblyMetadata{
        assert!(table.kind == MetadataTableKind::Assembly);
        return AssemblyMetadata{
            hash_alg:       HashAlgorithm::from_u32(table.decode_row_col(0,0)),
            major_version:  table.decode_row_col(0,1),
            minor_version:  table.decode_row_col(0,2),
            build_number:   table.decode_row_col(0,3),
            rev_number:     table.decode_row_col(0,4),
            flags:          AssemblyFlags{flags:table.decode_row_col(0,5)},
            public_key:     table.decode_row_col(0,6),
            name:           table.decode_row_col(0,7),
            culture:        table.decode_row_col(0,8),
            img:*img,
        };
    }
    pub fn from_image(img:&Image)->AssemblyMetadata{
        return Self::from_meta_table(&img.get_table_info(MetadataTableKind::Assembly),img);
    }
    pub fn get_name(&self)->String{
        return self.img.metadata_string_heap(self.name);
    }
    pub fn get_culture(&self)->String{
        return self.img.metadata_string_heap(self.culture);
    }
}
///Representation of assembly flags. More info <a href="https://docs.microsoft.com/en-us/dotnet/api/system.reflection.assemblyflags?view=net-6.0"> here </a>
pub struct AssemblyFlags{pub flags:u32}
impl AssemblyFlags{
    ///Checks is `WindowsRuntime` flag is set.
    pub fn is_set_WindowsRuntime(&self)->bool{
        return (self.flags & 512) != 0;
    }
    ///Checks is `Retargtable` flag is set.
    pub fn is_set_Retargtable(&self)->bool{
        return (self.flags & 256) != 0;
    }
    ///Checks if `PublicKey` flag is set.
    pub fn is_set_PublicKey(&self)->bool{
        return (self.flags & 1) != 0;
    }
    ///Checks if `DisableJitCompileOptimizer` flag is set.
    pub fn is_set_DisableJitCompileOptimizer(&self)->bool{
        return (self.flags & 16384) != 0;
    }
    ///Checks if `EnableJitCompileTracking` flag is set.
    pub fn is_set_EnableJitCompileTracking(&self)->bool{
        return (self.flags & 32768) != 0;
    }
    ///Returns the `ContentType` mask bits.
    pub fn content_type_mask(&self)->[bool;2]{
        return [(self.flags & 2048) != 0,(self.flags & 1024) != 0];
    }
}
impl std::fmt::Display for AssemblyFlags{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"AssemblyFlags{{WindowsRuntime:{}, ",self.is_set_WindowsRuntime())?;
        write!(f,"Retargtable:{}, ",                  self.is_set_Retargtable())?;
        write!(f,"PublicKey:{}, ",                    self.is_set_PublicKey())?;
        write!(f,"DisableJitCompileOptimizer:{}, ",   self.is_set_DisableJitCompileOptimizer())?;
        write!(f,"EnableJitCompileTracking:{}, ",     self.is_set_EnableJitCompileTracking())?;
        write!(f,"ContentTypeMask:{:?}}} ",           self.content_type_mask())
    }
}
///Assembly hash algotith type. More info <a href="docs.microsoft.com/en-us/dotnet/api/system.configuration.assemblies.assemblyhashalgorithm?view=net-6.0"> here </a>
#[repr(u32)]
pub enum HashAlgorithm{
    MD5     =   32771,
    None    =   0,
    SHA1    =   32772,
    SHA256  =   32780,
    SHA384  =   32781,
    SHA512  =   32782,
}
impl HashAlgorithm{
    fn from_u32(u:u32)->HashAlgorithm{
        return match u{
            32771   =>HashAlgorithm::MD5,
            0       =>HashAlgorithm::None,
            32772   =>HashAlgorithm::SHA1,
            32780   =>HashAlgorithm::SHA256,
            32781   =>HashAlgorithm::SHA384,
            32782   =>HashAlgorithm::SHA256,
            _=>panic!("{} is not a valid HashAlgorithm",u),
        };
    }
}
impl std::fmt::Display for HashAlgorithm{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        let s = match self{
            HashAlgorithm::MD5=>    "MD5",
            HashAlgorithm::None=>   "None",
            HashAlgorithm::SHA1=>   "SHA1",
            HashAlgorithm::SHA256=> "SHA1",
            HashAlgorithm::SHA384=> "SHA1",
            HashAlgorithm::SHA512=> "SHA1",
        };
        write!(f,"{}",s)
    }
}
impl std::fmt::Display for AssemblyMetadata{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"AssemblyMetadata{{")?;
        write!(f,"hash_alg:{}, ",        self.hash_alg)?;
        write!(f,"major_version:{}, ",   self.major_version)?;
        write!(f,"minor_version:{}, ",   self.minor_version)?;
        write!(f,"build_number:{}, ",    self.build_number)?;
        write!(f,"rev_number:{}, ",      self.rev_number)?;
        write!(f,"flags:{}, ",           self.flags)?;
        write!(f,"public_key:{}, ",      self.public_key)?;
        write!(f,"Name:{}, ",            self.get_name())?;
        write!(f,"Culture:{}}}",         self.get_culture())
    }
}