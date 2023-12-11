use crate::ut;

/// Abstract base types
impl ut::Type {
    /// A generic base type for most things (files, directories.)
    ///
    /// UTI: public.item
    #[doc(alias = "UTTypeItem")]
    #[inline]
    pub fn item() -> &'static Self {
        unsafe { UTTypeItem }
    }

    /// A base type for anything containing user-viewable document content
    /// (documents, pasteboard data, and document packages.)
    ///
    /// UTI: public.content
    #[doc(alias = "UTTypeContent")]
    #[inline]
    pub fn content() -> &'static Self {
        unsafe { UTTypeContent }
    }

    /// A base type for content formats supporting mixed embedded content
    /// (i.e., compound documents).
    ///
    /// UTI: public.composite-content
    /// conforms to: public.content
    #[doc(alias = "UTTypeCompositeContent")]
    #[inline]
    pub fn composite_content() -> &'static Self {
        unsafe { UTTypeCompositeContent }
    }

    /// A data item mountable as a volume
    ///
    /// UTI: public.disk-image
    #[doc(alias = "UTTypeDiskImage")]
    #[inline]
    pub fn disk_image() -> &'static Self {
        unsafe { UTTypeDiskImage }
    }
}
#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeItem: &'static ut::Type;
    static UTTypeContent: &'static ut::Type;
    static UTTypeCompositeContent: &'static ut::Type;
    static UTTypeDiskImage: &'static ut::Type;
}

/// Concrete base types
impl ut::Type {
    /// A base type for any sort of simple byte stream, including files and
    /// in-memory data.
    ///
    /// UTI: public.data
    /// conforms to: public.item
    #[doc(alias = "UTTypeData")]
    #[inline]
    pub fn data() -> &'static Self {
        unsafe { UTTypeData }
    }

    /// A file system directory (includes packages \em and folders.)
    ///
    /// UTI: public.directory
    /// conforms to: public.item
    #[doc(alias = "UTTypeDirectory")]
    #[inline]
    pub fn directory() -> &'static Self {
        unsafe { UTTypeDirectory }
    }

    /// Symbolic link and alias file types conform to this type.
    ///
    /// UTI: com.apple.resolvable
    #[doc(alias = "UTTypeResolvable")]
    #[inline]
    pub fn resolvable() -> &'static Self {
        unsafe { UTTypeResolvable }
    }

    /// A symbolic link.
    ///
    /// UTI: public.symlink
    /// conforms to: public.item, com.apple.resolvable
    #[doc(alias = "UTTypeSymbolicLink")]
    #[inline]
    pub fn symbolic_link() -> &'static Self {
        unsafe { UTTypeSymbolicLink }
    }

    /// An executable item.
    ///
    /// UTI: public.executable
    /// conforms to: public.item
    #[doc(alias = "UTTypeExecutable")]
    #[inline]
    pub fn executable() -> &'static Self {
        unsafe { UTTypeExecutable }
    }

    /// A volume mount point (resolvable, resolves to the root directory of a
    /// volume.)
    ///
    /// UTI: com.apple.mount-point
    /// conforms to: public.item, com.apple.resolvable
    #[doc(alias = "UTTypeMountPoint")]
    #[inline]
    pub fn mount_point() -> &'static Self {
        unsafe { UTTypeMountPoint }
    }

    /// A fully-formed alias file.
    ///
    /// UTI: com.apple.alias-file
    /// conforms to: public.data, com.apple.resolvable
    #[doc(alias = "UTTypeAliasFile")]
    #[inline]
    pub fn alias_file() -> &'static Self {
        unsafe { UTTypeAliasFile }
    }
}
#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeData: &'static ut::Type;
    static UTTypeDirectory: &'static ut::Type;
    static UTTypeResolvable: &'static ut::Type;
    static UTTypeSymbolicLink: &'static ut::Type;
    static UTTypeExecutable: &'static ut::Type;
    static UTTypeMountPoint: &'static ut::Type;
    static UTTypeAliasFile: &'static ut::Type;
    static UTTypeURLBookmarkData: &'static ut::Type;
}

/// URL types
impl ut::Type {
    /// A URL bookmark.
    ///
    /// UTI: com.apple.bookmark
    /// conforms to: public.data, com.apple.resolvable
    #[doc(alias = "UTTypeURLBookmarkData")]
    #[inline]
    pub fn url_bookmark_data() -> &'static Self {
        unsafe { UTTypeURLBookmarkData }
    }

    /// Any URL.
    ///
    /// UTI: public.url
    /// conforms to: public.data
    #[doc(alias = "UTTypeURL")]
    #[inline]
    pub fn url() -> &'static Self {
        unsafe { UTTypeURL }
    }

    /// A URL with the scheme \c "file:".
    ///
    /// UTI: public.file-url
    /// conforms to: public.url
    #[doc(alias = "UTTypeFileURL")]
    #[inline]
    pub fn file_url() -> &'static Self {
        unsafe { UTTypeFileURL }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeURL: &'static ut::Type;
    static UTTypeFileURL: &'static ut::Type;
}

/// Text types
impl ut::Type {
    /// The base type for all text-encoded data, including text with markup
    /// (HTML, RTF, etc.).
    ///
    /// UTI: public.text
    /// conforms to: public.data, public.content
    #[doc(alias = "UTTypeText")]
    #[inline]
    pub fn text() -> &'static Self {
        unsafe { UTTypeText }
    }

    /// Text with no markup and an unspecified encoding.
    ///
    /// UTI: public.plain-text
    /// conforms to: public.text
    #[doc(alias = "UTTypePlainText")]
    #[inline]
    pub fn plain_text() -> &'static Self {
        unsafe { UTTypePlainText }
    }

    /// Plain text encoded as UTF-8.
    ///
    /// UTI: public.utf8-plain-text
    /// conforms to: public.plain-text
    #[doc(alias = "UTTypeUTF8PlainText")]
    #[inline]
    pub fn utf8_plain_text() -> &'static Self {
        unsafe { UTTypeUTF8PlainText }
    }

    /// Plain text encoded as UTF-16 with a BOM, or if a BOM is not present,
    /// using "external representation" byte order (big-endian).
    ///
    /// UTI: public.utf16-external-plain-text
    /// conforms to: public.plain-text
    #[doc(alias = "UTTypeUTF16ExternalPlainText")]
    #[inline]
    pub fn utf16_external_plain_text() -> &'static Self {
        unsafe { UTTypeUTF16ExternalPlainText }
    }

    /// Plain text encoded as UTF-16, in native byte order, with an optional
    /// BOM.
    ///
    /// UTI: public.utf16-plain-text
    /// conforms to: public.plain-text
    #[doc(alias = "UTTypeUTF16PlainText")]
    #[inline]
    pub fn utf16_plain_text() -> &'static Self {
        unsafe { UTTypeUTF16PlainText }
    }

    /// Text containing delimited values.
    ///
    /// UTI: public.delimited-values-text
    /// conforms to: public.text
    #[doc(alias = "UTTypeDelimitedText")]
    #[inline]
    pub fn delimited_text() -> &'static Self {
        unsafe { UTTypeDelimitedText }
    }

    /// Text containing comma-separated values (.csv).
    ///
    /// UTI: public.comma-separated-values-text
    /// conforms to: public.delimited-values-text
    #[doc(alias = "UTTypeCommaSeparatedText")]
    #[inline]
    pub fn comma_separated_text() -> &'static Self {
        unsafe { UTTypeCommaSeparatedText }
    }

    /// Text containing tab-separated values.
    ///
    /// UTI: public.tab-separated-values-text
    /// conforms to: public.delimited-values-text
    #[doc(alias = "UTTypeTabSeparatedText")]
    #[inline]
    pub fn tab_separated_text() -> &'static Self {
        unsafe { UTTypeTabSeparatedText }
    }

    /// UTF-8 encoded text containing tab-separated values.
    ///
    /// UTI: public.utf8-tab-separated-values-text
    /// conforms to: public.tab-separated-values-text, public.utf8-plain-text
    #[doc(alias = "UTTypeUTF8TabSeparatedText")]
    #[inline]
    pub fn utf8_tab_separated_text() -> &'static Self {
        unsafe { UTTypeUTF8TabSeparatedText }
    }

    /// Rich Text Format data.
    ///
    /// UTI: public.rtf
    /// conforms to: public.text
    #[doc(alias = "UTTypeRTF")]
    #[inline]
    pub fn rtf() -> &'static Self {
        unsafe { UTTypeRTF }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeText: &'static ut::Type;
    static UTTypePlainText: &'static ut::Type;
    static UTTypeUTF8PlainText: &'static ut::Type;
    static UTTypeUTF16ExternalPlainText: &'static ut::Type;
    static UTTypeUTF16PlainText: &'static ut::Type;
    static UTTypeDelimitedText: &'static ut::Type;
    static UTTypeCommaSeparatedText: &'static ut::Type;
    static UTTypeTabSeparatedText: &'static ut::Type;
    static UTTypeUTF8TabSeparatedText: &'static ut::Type;
    static UTTypeRTF: &'static ut::Type;
}

/// Markup languages
impl ut::Type {
    /// Any version of HTML.
    ///
    /// UTI: public.html
    /// conforms to: public.text
    #[doc(alias = "UTTypeHTML")]
    #[inline]
    pub fn html() -> &'static Self {
        unsafe { UTTypeHTML }
    }

    /// Generic XML.
    ///
    /// UTI: public.xml
    /// conforms to: public.text
    #[doc(alias = "UTTypeXML")]
    #[inline]
    pub fn xml() -> &'static Self {
        unsafe { UTTypeXML }
    }

    /// Yet Another Markup Language.
    ///
    /// UTI: public.yaml
    /// conforms to: public.text
    #[doc(alias = "UTTypeYAML")]
    #[inline]
    pub fn yaml() -> &'static Self {
        unsafe { UTTypeYAML }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeHTML: &'static ut::Type;
    static UTTypeXML: &'static ut::Type;
    static UTTypeYAML: &'static ut::Type;
}

/// Programming languages
impl ut::Type {
    /// Abstract type for source code of any language.
    ///
    /// UTI: public.source-code
    /// conforms to: public.plain-text
    #[doc(alias = "UTTypeSourceCode")]
    #[inline]
    pub fn src_code() -> &'static Self {
        unsafe { UTTypeSourceCode }
    }

    /// Assembly language source (.s)
    ///
    /// UTI: public.assembly-source
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeAssemblyLanguageSource")]
    #[inline]
    pub fn asm_src() -> &'static Self {
        unsafe { UTTypeAssemblyLanguageSource }
    }

    /// C source code (.c)
    ///
    /// UTI: public.c-source
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeCSource")]
    #[inline]
    pub fn c_src() -> &'static Self {
        unsafe { UTTypeCSource }
    }

    /// Objective-C source code (.m)
    ///
    /// UTI: public.objective-c-source
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeObjectiveCSource")]
    #[inline]
    pub fn obj_c_src() -> &'static Self {
        unsafe { UTTypeObjectiveCSource }
    }

    /// Swift source code (.swift)
    ///
    /// UTI: public.swift-source
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeSwiftSource")]
    #[inline]
    pub fn swift_src() -> &'static Self {
        unsafe { UTTypeSwiftSource }
    }

    /// C++ source code (.cp, etc.)
    ///
    /// UTI: public.c-plus-plus-source
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeCPlusPlusSource")]
    #[inline]
    pub fn cpp_src() -> &'static Self {
        unsafe { UTTypeCPlusPlusSource }
    }
    /// Objective-C++ source code.
    ///
    /// UTI: public.objective-c-plus-plus-source
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeObjectiveCPlusPlusSource")]
    #[inline]
    pub fn obj_cpp_src() -> &'static Self {
        unsafe { UTTypeObjectiveCPlusPlusSource }
    }

    /// A C header.
    ///
    /// UTI: public.c-header
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeCHeader")]
    #[inline]
    pub fn c_header() -> &'static Self {
        unsafe { UTTypeCHeader }
    }

    /// A C++ header.
    ///
    /// UTI: public.c-plus-plus-header
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeCPlusPlusHeader")]
    #[inline]
    pub fn cpp_header() -> &'static Self {
        unsafe { UTTypeCPlusPlusHeader }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeSourceCode: &'static ut::Type;
    static UTTypeAssemblyLanguageSource: &'static ut::Type;
    static UTTypeCSource: &'static ut::Type;
    static UTTypeObjectiveCSource: &'static ut::Type;
    static UTTypeSwiftSource: &'static ut::Type;
    static UTTypeCPlusPlusSource: &'static ut::Type;
    static UTTypeObjectiveCPlusPlusSource: &'static ut::Type;
    static UTTypeCHeader: &'static ut::Type;
    static UTTypeCPlusPlusHeader: &'static ut::Type;
}

/// Scripting languages
impl ut::Type {
    /// A base type for any scripting language source.
    ///
    /// UTI: public.script
    /// conforms to: public.source-code
    #[doc(alias = "UTTypeScript")]
    #[inline]
    pub fn script() -> &'static Self {
        unsafe { UTTypeScript }
    }

    /// An AppleScript text-based script (.applescript).
    ///
    /// UTI: com.apple.applescript.text
    /// conforms to: public.script
    #[doc(alias = "UTTypeAppleScript")]
    #[inline]
    pub fn apple_script() -> &'static Self {
        unsafe { UTTypeAppleScript }
    }

    /// An Open Scripting Architecture binary script (.scpt).
    ///
    /// UTI: com.apple.applescript.script
    /// conforms to: public.data, public.script
    #[doc(alias = "UTTypeOSAScript")]
    #[inline]
    pub fn osa_script() -> &'static Self {
        unsafe { UTTypeOSAScript }
    }

    /// An Open Scripting Architecture script bundle (.scptd).
    ///
    /// UTI: com.apple.applescript.script-bundle
    /// conforms to: com.apple.bundle, com.apple.package, public.script
    #[doc(alias = "UTTypeOSAScriptBundle")]
    #[inline]
    pub fn osa_script_bundle() -> &'static Self {
        unsafe { UTTypeOSAScriptBundle }
    }

    /// JavaScript source code
    ///
    /// UTI: com.netscape.javascript-source
    /// conforms to: public.source-code, public.executable
    #[doc(alias = "UTTypeJavaScript")]
    #[inline]
    pub fn java_script() -> &'static Self {
        unsafe { UTTypeJavaScript }
    }

    /// The base type for shell scripts.
    ///
    /// UTI: public.shell-script
    /// conforms to: public.script
    #[doc(alias = "UTTypeShellScript")]
    #[inline]
    pub fn shell_script() -> &'static Self {
        unsafe { UTTypeShellScript }
    }

    /// A Perl script.
    ///
    /// UTI: public.perl-script
    /// conforms to: public.shell-script
    #[doc(alias = "UTTypePerlScript")]
    #[inline]
    pub fn perl_script() -> &'static Self {
        unsafe { UTTypePerlScript }
    }

    /// A Python script.
    ///
    /// UTI: public.python-script
    /// conforms to: public.shell-script
    #[doc(alias = "UTTypePythonScript")]
    #[inline]
    pub fn python_script() -> &'static Self {
        unsafe { UTTypePythonScript }
    }

    /// A Ruby script.
    ///
    /// UTI: public.ruby-script
    /// conforms to: public.shell-script
    #[doc(alias = "UTTypeRubyScript")]
    #[inline]
    pub fn ruby_script() -> &'static Self {
        unsafe { UTTypeRubyScript }
    }

    /// A PHP script.
    ///
    /// UTI: public.php-script
    /// conforms to: public.shell-script
    #[doc(alias = "UTTypePHPScript")]
    #[inline]
    pub fn php_script() -> &'static Self {
        unsafe { UTTypePHPScript }
    }

    /// A makefile.
    ///
    /// UTI: public.make-source
    /// conforms to: public.script
    #[doc(alias = "UTTypeMakefile")]
    #[inline]
    pub fn makefile() -> &'static Self {
        unsafe { UTTypeMakefile }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeScript: &'static ut::Type;
    static UTTypeAppleScript: &'static ut::Type;
    static UTTypeOSAScript: &'static ut::Type;
    static UTTypeOSAScriptBundle: &'static ut::Type;
    static UTTypeJavaScript: &'static ut::Type;
    static UTTypeShellScript: &'static ut::Type;
    static UTTypePerlScript: &'static ut::Type;
    static UTTypePythonScript: &'static ut::Type;
    static UTTypeRubyScript: &'static ut::Type;
    static UTTypePHPScript: &'static ut::Type;
    static UTTypeMakefile: &'static ut::Type;
}

/// Serialized data types
impl ut::Type {
    /// JavaScript object notation (JSON) data
    ///
    /// UTI: public.json
    /// conforms to: public.text
    /// JSON almost (but doesn't quite) conforms to
    /// com.netscape.javascript-source.
    #[doc(alias = "UTTypeJSON")]
    #[inline]
    pub fn json() -> &'static Self {
        unsafe { UTTypeJSON }
    }

    /// A base type for property lists.
    ///
    /// UTI: com.apple.property-list
    /// conforms to: public.data
    #[doc(alias = "UTTypePropertyList")]
    #[inline]
    pub fn prop_list() -> &'static Self {
        unsafe { UTTypePropertyList }
    }

    /// An XML property list.
    ///
    /// UTI: com.apple.xml-property-list
    /// conforms to: public.xml, com.apple.property-list
    #[doc(alias = "UTTypeXMLPropertyList")]
    #[inline]
    pub fn xml_prop_list() -> &'static Self {
        unsafe { UTTypeXMLPropertyList }
    }

    /// A binary property list.
    ///
    /// UTI: com.apple.binary-property-list
    /// conforms to: com.apple.property-list
    #[doc(alias = "UTTypeBinaryPropertyList")]
    #[inline]
    pub fn binary_prop_list() -> &'static Self {
        unsafe { UTTypeBinaryPropertyList }
    }
}
#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeJSON: &'static ut::Type;
    static UTTypePropertyList: &'static ut::Type;
    static UTTypeXMLPropertyList: &'static ut::Type;
    static UTTypeBinaryPropertyList: &'static ut::Type;
}

/// Composite content types
impl ut::Type {
    /// An Adobe PDF document.
    ///
    /// UTI: com.adobe.pdf
    /// conforms to: public.data, public.composite-content
    #[doc(alias = "UTTypePDF")]
    #[inline]
    pub fn pdf() -> &'static Self {
        unsafe { UTTypePDF }
    }

    /// A Rich Text Format Directory document (RTF with content embedding
    /// in its on-disk format.)
    ///
    /// UTI: com.apple.rtfd
    /// conforms to: com.apple.package, public.composite-content
    #[doc(alias = "UTTypeRTFD")]
    #[inline]
    pub fn rtfd() -> &'static Self {
        unsafe { UTTypeRTFD }
    }

    /// A flattened RTFD document (formatted for the pasteboard.)
    ///
    /// UTI: com.apple.flat-rtfd
    /// conforms to: public.data, public.composite-content
    #[doc(alias = "UTTypeFlatRTFD")]
    #[inline]
    pub fn flat_rtfd() -> &'static Self {
        unsafe { UTTypeFlatRTFD }
    }

    /// The WebKit webarchive format.
    ///
    /// UTI: com.apple.webarchive
    /// conforms to: public.data, public.composite-content
    #[doc(alias = "UTTypeWebArchive")]
    #[inline]
    pub fn web_archive() -> &'static Self {
        unsafe { UTTypeWebArchive }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypePDF: &'static ut::Type;
    static UTTypeRTFD: &'static ut::Type;
    static UTTypeFlatRTFD: &'static ut::Type;
    static UTTypeWebArchive: &'static ut::Type;
}

/// Image content types
impl ut::Type {
    /// A base type for abstract image data.
    ///
    /// UTI: public.image
    /// conforms to: public.data, public.content
    #[doc(alias = "UTTypeImage")]
    #[inline]
    pub fn image() -> &'static Self {
        unsafe { UTTypeImage }
    }

    /// A JPEG image.
    ///
    /// UTI: public.jpeg
    /// conforms to: public.image
    #[doc(alias = "UTTypeJPEG")]
    #[inline]
    pub fn jpeg() -> &'static Self {
        unsafe { UTTypeJPEG }
    }

    /// A TIFF image.
    ///
    /// UTI: public.tiff
    /// conforms to: public.image
    #[doc(alias = "UTTypeTIFF")]
    #[inline]
    pub fn tiff() -> &'static Self {
        unsafe { UTTypeTIFF }
    }

    /// A GIF image.
    ///
    /// UTI: com.compuserve.gif
    /// conforms to: public.image
    #[doc(alias = "UTTypeGIF")]
    #[inline]
    pub fn gif() -> &'static Self {
        unsafe { UTTypeGIF }
    }

    /// A PNG image.
    ///
    /// UTI: public.png
    /// conforms to: public.image
    #[doc(alias = "UTTypePNG")]
    #[inline]
    pub fn png() -> &'static Self {
        unsafe { UTTypePNG }
    }

    /// Apple icon data
    ///
    /// UTI: com.apple.icns
    /// conforms to: public.image
    #[doc(alias = "UTTypeICNS")]
    #[inline]
    pub fn icns() -> &'static Self {
        unsafe { UTTypeICNS }
    }

    /// A Windows bitmap.
    ///
    /// UTI: com.microsoft.bmp
    /// conforms to: public.image
    #[doc(alias = "UTTypeBMP")]
    #[inline]
    pub fn bmp() -> &'static Self {
        unsafe { UTTypeBMP }
    }

    /// Windows icon data
    ///
    /// UTI: com.microsoft.ico
    /// conforms to: public.image
    #[doc(alias = "UTTypeICO")]
    #[inline]
    pub fn ico() -> &'static Self {
        unsafe { UTTypeICO }
    }

    /// A base type for raw image data (.raw).
    ///
    /// UTI: public.camera-raw-image
    /// conforms to: public.image
    #[doc(alias = "UTTypeRAWImage")]
    #[inline]
    pub fn raw_image() -> &'static Self {
        unsafe { UTTypeRAWImage }
    }

    /// A Scalable Vector Graphics image.
    ///
    /// UTI: public.svg-image
    /// conforms to: public.image
    #[doc(alias = "UTTypeSVG")]
    #[inline]
    pub fn svg() -> &'static Self {
        unsafe { UTTypeSVG }
    }

    /// A Live Photo.
    ///
    /// UTI: com.apple.live-photo
    #[doc(alias = "UTTypeLivePhoto")]
    #[inline]
    pub fn live_photo() -> &'static Self {
        unsafe { UTTypeLivePhoto }
    }

    /// A High Efficiency Image File Format image.
    ///
    /// UTI: public.heif
    /// conforms to: public.heif-standard
    #[doc(alias = "UTTypeHEIF")]
    #[inline]
    pub fn heif() -> &'static Self {
        unsafe { UTTypeHEIF }
    }

    /// A High Efficiency Image Coding image.
    ///
    /// UTI: public.heic
    /// conforms to: public.heif-standard
    #[doc(alias = "UTTypeHEIC")]
    #[inline]
    pub fn heic() -> &'static Self {
        unsafe { UTTypeHEIC }
    }

    /// The WebP image format.
    ///
    /// UTI: org.webmproject.webp
    /// conforms to: public.image
    #[doc(alias = "UTTypeWebP")]
    #[inline]
    pub fn webp() -> &'static Self {
        unsafe { UTTypeWebP }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeImage: &'static ut::Type;
    static UTTypeJPEG: &'static ut::Type;
    static UTTypeTIFF: &'static ut::Type;
    static UTTypeGIF: &'static ut::Type;
    static UTTypePNG: &'static ut::Type;
    static UTTypeICNS: &'static ut::Type;
    static UTTypeBMP: &'static ut::Type;
    static UTTypeICO: &'static ut::Type;
    static UTTypeRAWImage: &'static ut::Type;
    static UTTypeSVG: &'static ut::Type;
    static UTTypeLivePhoto: &'static ut::Type;
    static UTTypeHEIF: &'static ut::Type;
    static UTTypeHEIC: &'static ut::Type;
    static UTTypeWebP: &'static ut::Type;
}

/// 3D Content
impl ut::Type {
    /// A base type for 3D content.
    ///
    /// UTI: public.3d-content
    /// conforms to: public.content
    #[doc(alias = "UTType3DContent")]
    #[inline]
    pub fn _3d_content() -> &'static Self {
        unsafe { UTType3DContent }
    }

    /// Universal Scene Description content.
    ///
    /// UTI: com.pixar.universal-scene-description
    /// conforms to: public.3d-content, public.data
    #[doc(alias = "UTTypeUSD")]
    #[inline]
    pub fn usd() -> &'static Self {
        unsafe { UTTypeUSD }
    }

    /// Universal Scene Description Package content.
    ///
    /// UTI: com.pixar.universal-scene-description-mobile
    /// conforms to: public.3d-content, public.data
    #[doc(alias = "UTTypeUSDZ")]
    #[inline]
    pub fn usdz() -> &'static Self {
        unsafe { UTTypeUSDZ }
    }

    /// A Reality File.
    ///
    /// UTI: com.apple.reality
    /// conforms to: public.data
    #[doc(alias = "UTTypeRealityFile")]
    #[inline]
    pub fn reality_file() -> &'static Self {
        unsafe { UTTypeRealityFile }
    }

    /// A SceneKit serialized scene.
    ///
    /// UTI: com.apple.scenekit.scene
    /// conforms to: public.3d-content, public.data
    #[doc(alias = "UTTypeSceneKitScene")]
    #[inline]
    pub fn sk_scene() -> &'static Self {
        unsafe { UTTypeSceneKitScene }
    }

    /// An AR reference object.
    ///
    /// UTI: com.apple.arobject
    /// conforms to: public.data
    #[doc(alias = "UTTypeARReferenceObject")]
    #[inline]
    pub fn ar_reference_obj() -> &'static Self {
        unsafe { UTTypeARReferenceObject }
    }

    /// Any audio and/or video content.
    ///
    /// UTI: public.audiovisual-content
    /// conforms to: public.data, public.content
    #[doc(alias = "UTTypeAudiovisualContent")]
    #[inline]
    pub fn av_content() -> &'static Self {
        unsafe { UTTypeAudiovisualContent }
    }

    /// A media format which may contain both video and audio.
    ///
    /// This type corresponds to what users would label a "movie".
    ///
    /// UTI: public.movie
    /// conforms to: public.audiovisual-content
    #[doc(alias = "UTTypeMovie")]
    #[inline]
    pub fn movie() -> &'static Self {
        unsafe { UTTypeMovie }
    }

    /// Pure video data with no audio data.
    ///
    /// UTI: public.video
    /// conforms to: public.movie
    #[doc(alias = "UTTypeVideo")]
    #[inline]
    pub fn video() -> &'static Self {
        unsafe { UTTypeVideo }
    }

    /// Pure audio data with no video data.
    ///
    /// UTI: public.audio
    /// conforms to: public.audiovisual-content
    #[doc(alias = "UTTypeAudio")]
    #[inline]
    pub fn audio() -> &'static Self {
        unsafe { UTTypeAudio }
    }

    /// A QuickTime movie.
    ///
    /// UTI: com.apple.quicktime-movie
    /// conforms to: public.movie
    #[doc(alias = "UTTypeQuickTimeMovie")]
    #[inline]
    pub fn quick_time_movie() -> &'static Self {
        unsafe { UTTypeQuickTimeMovie }
    }

    /// An MPEG-1 or MPEG-2 movie.
    ///
    /// UTI: public.mpeg
    /// conforms to: public.movie
    #[doc(alias = "UTTypeMPEG")]
    #[inline]
    pub fn mpeg() -> &'static Self {
        unsafe { UTTypeMPEG }
    }

    /// An MPEG-2 video.
    ///
    /// UTI: public.mpeg-2-video
    /// conforms to: public.video
    #[doc(alias = "UTTypeMPEG2Video")]
    #[inline]
    pub fn mpeg2video() -> &'static Self {
        unsafe { UTTypeMPEG2Video }
    }

    /// The MPEG-2 Transport Stream movie format.
    ///
    /// UTI: public.mpeg-2-transport-stream
    /// conforms to: public.movie
    #[doc(alias = "UTTypeMPEG2TransportStream")]
    #[inline]
    pub fn mpeg2ts() -> &'static Self {
        unsafe { UTTypeMPEG2TransportStream }
    }

    /// MP3 audio.
    ///
    /// UTI: public.mp3
    /// conforms to: public.audio
    #[doc(alias = "UTTypeMP3")]
    #[inline]
    pub fn mp3() -> &'static Self {
        unsafe { UTTypeMP3 }
    }

    /// MPEG-4 movie
    ///
    /// UTI: public.mpeg-4
    /// conforms to: public.movie
    #[doc(alias = "UTTypeMPEG4Movie")]
    #[inline]
    pub fn mpeg4video() -> &'static Self {
        unsafe { UTTypeMPEG4Movie }
    }

    /// An MPEG-4 audio layer file.
    ///
    /// UTI: public.mpeg-4-audio
    /// conforms to: public.mpeg-4, public.audio
    #[doc(alias = "UTTypeMPEG4Audio")]
    #[inline]
    pub fn mpeg4audio() -> &'static Self {
        unsafe { UTTypeMPEG4Audio }
    }

    /// The Apple protected MPEG4 format (.m4p, iTunes music store format.)
    ///
    /// UTI: com.apple.protected-mpeg-4-audio
    /// conforms to: public.audio
    #[doc(alias = "UTTypeAppleProtectedMPEG4Audio")]
    #[inline]
    pub fn apple_protected_mpeg4audio() -> &'static Self {
        unsafe { UTTypeAppleProtectedMPEG4Audio }
    }

    /// An Apple protected MPEG-4 movie.
    ///
    /// UTI: com.apple.protected-mpeg-4-video
    /// conforms to: com.apple.m4v-video
    #[doc(alias = "UTTypeAppleProtectedMPEG4Video")]
    #[inline]
    pub fn apple_protected_mpeg4video() -> &'static Self {
        unsafe { UTTypeAppleProtectedMPEG4Video }
    }

    /// The AVI movie format.
    ///
    /// UTI: public.avi
    /// conforms to: public.movie
    #[doc(alias = "UTTypeAVI")]
    #[inline]
    pub fn avi() -> &'static Self {
        unsafe { UTTypeAVI }
    }

    /// The AIFF audio format
    ///
    /// UTI: public.aiff-audio
    /// conforms to: public.aifc-audio
    #[doc(alias = "UTTypeAIFF")]
    #[inline]
    pub fn aiff() -> &'static Self {
        unsafe { UTTypeAIFF }
    }

    /// The Microsoft waveform audio format (.wav).
    ///
    /// UTI: com.microsoft.waveform-audio
    /// conforms to: public.audio
    #[doc(alias = "UTTypeWAV")]
    #[inline]
    pub fn wav() -> &'static Self {
        unsafe { UTTypeWAV }
    }

    /// The MIDI audio format.
    ///
    /// UTI: public.midi-audio
    /// conforms to: public.audio
    #[doc(alias = "UTTypeMIDI")]
    #[inline]
    pub fn midi() -> &'static Self {
        unsafe { UTTypeMIDI }
    }

    /// The base type for playlists.
    ///
    /// UTI: public.playlist
    #[doc(alias = "UTTypePlaylist")]
    #[inline]
    pub fn playlist() -> &'static Self {
        unsafe { UTTypePlaylist }
    }

    /// An M3U or M3U8 playlist
    ///
    /// UTI: public.m3u-playlist
    /// conforms to: public.text, public.playlist
    #[doc(alias = "UTTypeM3UPlaylist")]
    #[inline]
    pub fn m3u_playlist() -> &'static Self {
        unsafe { UTTypeM3UPlaylist }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTType3DContent: &'static ut::Type;
    static UTTypeUSD: &'static ut::Type;
    static UTTypeUSDZ: &'static ut::Type;
    static UTTypeRealityFile: &'static ut::Type;
    static UTTypeSceneKitScene: &'static ut::Type;
    static UTTypeARReferenceObject: &'static ut::Type;
    static UTTypeAudiovisualContent: &'static ut::Type;
    static UTTypeMovie: &'static ut::Type;
    static UTTypeVideo: &'static ut::Type;
    static UTTypeAudio: &'static ut::Type;
    static UTTypeQuickTimeMovie: &'static ut::Type;
    static UTTypeMPEG: &'static ut::Type;
    static UTTypeMPEG2Video: &'static ut::Type;
    static UTTypeMPEG2TransportStream: &'static ut::Type;
    static UTTypeMP3: &'static ut::Type;
    static UTTypeMPEG4Movie: &'static ut::Type;
    static UTTypeMPEG4Audio: &'static ut::Type;
    static UTTypeAppleProtectedMPEG4Audio: &'static ut::Type;
    static UTTypeAppleProtectedMPEG4Video: &'static ut::Type;
    static UTTypeAVI: &'static ut::Type;
    static UTTypeAIFF: &'static ut::Type;
    static UTTypeWAV: &'static ut::Type;
    static UTTypeMIDI: &'static ut::Type;
    static UTTypePlaylist: &'static ut::Type;
    static UTTypeM3UPlaylist: &'static ut::Type;
}

/// Directory types
impl ut::Type {
    /// A user-browsable directory (i.e. not a package.)
    ///
    /// UTI: public.folder
    /// conforms to: public.directory
    #[doc(alias = "UTTypeFolder")]
    #[inline]
    pub fn folder() -> &'static Self {
        unsafe { UTTypeFolder }
    }

    /// The root folder of a volume or mount point.
    ///
    /// UTI: public.volume
    /// conforms to: public.folder
    #[doc(alias = "UTTypeVolume")]
    #[inline]
    pub fn volume() -> &'static Self {
        unsafe { UTTypeVolume }
    }

    /// A packaged directory.
    ///
    /// UTI: com.apple.package
    /// conforms to: public.directory
    #[doc(alias = "UTTypePackage")]
    #[inline]
    pub fn package() -> &'static Self {
        unsafe { UTTypePackage }
    }

    /// A directory conforming to one of the \c CFBundle layouts.
    ///
    /// UTI: com.apple.bundle
    /// conforms to: public.directory
    #[doc(alias = "UTTypeBundle")]
    #[inline]
    pub fn bundle() -> &'static Self {
        unsafe { UTTypeBundle }
    }

    /// The base type for bundle-based plugins.
    ///
    /// UTI: com.apple.plugin
    /// conforms to: com.apple.bundle, com.apple.package
    #[doc(alias = "UTTypePluginBundle")]
    #[inline]
    pub fn plugin_bundle() -> &'static Self {
        unsafe { UTTypePluginBundle }
    }

    /// A Spotlight metadata importer bundle.
    ///
    /// UTI: com.apple.metadata-importer
    /// conforms to: com.apple.plugin
    #[doc(alias = "UTTypeSpotlightImporter")]
    #[inline]
    pub fn spotlight_importer() -> &'static Self {
        unsafe { UTTypeSpotlightImporter }
    }

    /// A QuickLook preview generator bundle.
    ///
    /// UTI: com.apple.quicklook-generator
    /// conforms to: com.apple.plugin
    #[doc(alias = "UTTypeQuickLookGenerator")]
    #[inline]
    pub fn quick_look_generator() -> &'static Self {
        unsafe { UTTypeQuickLookGenerator }
    }

    /// An XPC service bundle.
    ///
    /// UTI: com.apple.xpc-service
    /// conforms to: com.apple.bundle, com.apple.package
    #[doc(alias = "UTTypeXPCService")]
    #[inline]
    pub fn xpc_service() -> &'static Self {
        unsafe { UTTypeXPCService }
    }

    /// A macOS or iOS framework bundle.
    ///
    /// UTI: com.apple.framework
    /// conforms to: com.apple.bundle
    #[doc(alias = "UTTypeFramework")]
    #[inline]
    pub fn framework() -> &'static Self {
        unsafe { UTTypeFramework }
    }

    /// The base type for macOS and iOS applications.
    ///
    /// UTI: com.apple.application
    /// conforms to: public.executable
    #[doc(alias = "UTTypeApplication")]
    #[inline]
    pub fn app() -> &'static Self {
        unsafe { UTTypeApplication }
    }

    /// A bundled application.
    ///
    /// UTI: com.apple.application-bundle
    /// conforms to: com.apple.application, com.apple.bundle, com.apple.package
    #[doc(alias = "UTTypeApplicationBundle")]
    #[inline]
    pub fn app_bundle() -> &'static Self {
        unsafe { UTTypeApplicationBundle }
    }

    /// An application extension (.appex).
    ///
    /// UTI: com.apple.application-and-system-extension
    /// conforms to: com.apple.xpc-service
    #[doc(alias = "UTTypeApplicationExtension")]
    #[inline]
    pub fn appex() -> &'static Self {
        unsafe { UTTypeApplicationExtension }
    }

    /// A UNIX executable (flat file.)
    ///
    /// UTI: public.unix-executable
    /// conforms to: public.data, public.executable
    #[doc(alias = "UTTypeUnixExecutable")]
    #[inline]
    pub fn unix_executable() -> &'static Self {
        unsafe { UTTypeUnixExecutable }
    }

    /// A Windows executable (.exe).
    ///
    /// UTI: com.microsoft.windows-executable
    /// conforms to: public.data, public.executable
    #[doc(alias = "UTTypeEXE")]
    #[inline]
    pub fn exe() -> &'static Self {
        unsafe { UTTypeEXE }
    }

    /// A System Preferences pane.
    ///
    /// UTI: com.apple.systempreference.prefpane
    /// conforms to: com.apple.package, com.apple.bundle
    #[doc(alias = "UTTypeSystemPreferencesPane")]
    #[inline]
    pub fn sys_pref_pane() -> &'static Self {
        unsafe { UTTypeSystemPreferencesPane }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeFolder: &'static ut::Type;
    static UTTypeVolume: &'static ut::Type;
    static UTTypePackage: &'static ut::Type;
    static UTTypeBundle: &'static ut::Type;
    static UTTypePluginBundle: &'static ut::Type;
    static UTTypeSpotlightImporter: &'static ut::Type;
    static UTTypeQuickLookGenerator: &'static ut::Type;
    static UTTypeXPCService: &'static ut::Type;
    static UTTypeFramework: &'static ut::Type;
    static UTTypeApplication: &'static ut::Type;
    static UTTypeApplicationBundle: &'static ut::Type;
    static UTTypeApplicationExtension: &'static ut::Type;
    static UTTypeUnixExecutable: &'static ut::Type;
    static UTTypeEXE: &'static ut::Type;
    static UTTypeSystemPreferencesPane: &'static ut::Type;
}

/// Archival and compression types
impl ut::Type {
    /// An archive of files and directories
    ///
    /// UTI: public.archive
    #[doc(alias = "UTTypeArchive")]
    #[inline]
    pub fn archive() -> &'static Self {
        unsafe { UTTypeArchive }
    }

    /// A GNU zip archive.
    ///
    /// UTI: org.gnu.gnu-zip-archive
    /// conforms to: public.data, public.archive
    #[doc(alias = "UTTypeGZIP")]
    #[inline]
    pub fn gzip() -> &'static Self {
        unsafe { UTTypeGZIP }
    }

    /// A bzip2 archive.
    ///
    /// UTI: public.bzip2-archive
    /// conforms to: public.data, public.archive
    #[doc(alias = "UTTypeBZ2")]
    #[inline]
    pub fn bz2() -> &'static Self {
        unsafe { UTTypeBZ2 }
    }

    /// A zip archive.
    ///
    /// UTI: public.zip-archive
    /// conforms to: com.pkware.zip-archive
    #[doc(alias = "UTTypeZIP")]
    #[inline]
    pub fn zip() -> &'static Self {
        unsafe { UTTypeZIP }
    }

    /// An Apple Archive.
    ///
    /// UTI: com.apple.archive
    /// conforms to: public.data, public.archive
    #[doc(alias = "UTTypeAppleArchive")]
    #[inline]
    pub fn apple_archive() -> &'static Self {
        unsafe { UTTypeAppleArchive }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeArchive: &'static ut::Type;
    static UTTypeGZIP: &'static ut::Type;
    static UTTypeBZ2: &'static ut::Type;
    static UTTypeZIP: &'static ut::Type;
    static UTTypeAppleArchive: &'static ut::Type;
}

/// Document types
impl ut::Type {
    /// A base type for spreadsheet documents.
    ///
    /// UTI: public.spreadsheet
    /// conforms to: public.content
    #[doc(alias = "UTTypeSpreadsheet")]
    #[inline]
    pub fn spreadsheet() -> &'static Self {
        unsafe { UTTypeSpreadsheet }
    }

    /// A base type for presentation documents.
    ///
    /// UTI: public.presentation
    /// conforms to: public.composite-content
    #[doc(alias = "UTTypePresentation")]
    #[inline]
    pub fn presentation() -> &'static Self {
        unsafe { UTTypePresentation }
    }

    /// A database store.
    ///
    /// UTI: public.database
    #[doc(alias = "UTTypeDatabase")]
    #[inline]
    pub fn database() -> &'static Self {
        unsafe { UTTypeDatabase }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeSpreadsheet: &'static ut::Type;
    static UTTypePresentation: &'static ut::Type;
    static UTTypeDatabase: &'static ut::Type;
}

/// Messages, contacts, and calendar types
impl ut::Type {
    /// A base type for messages (email, IM, etc.)
    ///
    /// UTI: public.message
    #[doc(alias = "UTTypeMessage")]
    #[inline]
    pub fn message() -> &'static Self {
        unsafe { UTTypeMessage }
    }

    /// Contact information, e.g. for a person, group, organization
    ///
    /// UTI: public.contact
    #[doc(alias = "UTTypeContact")]
    #[inline]
    pub fn contact() -> &'static Self {
        unsafe { UTTypeContact }
    }

    /// A vCard file.
    ///
    /// UTI: public.vcard
    /// conforms to: public.text, public.contact
    #[doc(alias = "UTTypeVCard")]
    #[inline]
    pub fn v_card() -> &'static Self {
        unsafe { UTTypeVCard }
    }

    /// A to-do item.
    ///
    /// UTI: public.to-do-item
    #[doc(alias = "UTTypeToDoItem")]
    #[inline]
    pub fn todo_item() -> &'static Self {
        unsafe { UTTypeToDoItem }
    }

    /// A calendar event.
    ///
    /// UTI: public.calendar-event
    #[doc(alias = "UTTypeCalendarEvent")]
    #[inline]
    pub fn calendar_event() -> &'static Self {
        unsafe { UTTypeCalendarEvent }
    }

    /// An e-mail message.
    ///
    /// UTI: public.email-message
    /// conforms to: public.message
    #[doc(alias = "UTTypeEmailMessage")]
    #[inline]
    pub fn email_message() -> &'static Self {
        unsafe { UTTypeEmailMessage }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeMessage: &'static ut::Type;
    static UTTypeContact: &'static ut::Type;
    static UTTypeVCard: &'static ut::Type;
    static UTTypeToDoItem: &'static ut::Type;
    static UTTypeCalendarEvent: &'static ut::Type;
    static UTTypeEmailMessage: &'static ut::Type;
}

/// Internet locations
impl ut::Type {
    /// A base type for Apple Internet location files.
    ///
    /// UTI: com.apple.internet-location
    /// conforms to: public.data
    #[doc(alias = "UTTypeInternetLocation")]
    #[inline]
    pub fn internet_location() -> &'static Self {
        unsafe { UTTypeInternetLocation }
    }

    /// Microsoft Internet shortcut files (.url).
    ///
    /// UTI: com.apple.internet-location
    /// conforms to: public.data
    #[doc(alias = "UTTypeInternetShortcut")]
    #[inline]
    pub fn internet_shortcut() -> &'static Self {
        unsafe { UTTypeInternetShortcut }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeInternetLocation: &'static ut::Type;
    static UTTypeInternetShortcut: &'static ut::Type;
}

/// Miscellaneous types
impl ut::Type {
    /// A base type for fonts.
    ///
    /// UTI: public.font
    #[doc(alias = "UTTypeFont")]
    #[inline]
    pub fn font() -> &'static Self {
        unsafe { UTTypeFont }
    }

    /// A bookmark.
    ///
    /// UTI: public.bookmark
    #[doc(alias = "UTTypeBookmark")]
    #[inline]
    pub fn bookmark() -> &'static Self {
        unsafe { UTTypeBookmark }
    }

    /// PKCS#12 data.
    ///
    /// UTI: com.rsa.pkcs-12
    /// conforms to: public.data
    #[doc(alias = "UTTypePKCS12")]
    #[inline]
    pub fn pkcs12() -> &'static Self {
        unsafe { UTTypePKCS12 }
    }

    /// An X.509 certificate.
    ///
    /// UTI: public.x509-certificate
    /// conforms to: public.data
    #[doc(alias = "UTTypeX509Certificate")]
    #[inline]
    pub fn x509cert() -> &'static Self {
        unsafe { UTTypeX509Certificate }
    }

    /// The EPUB format.
    ///
    /// UTI: org.idpf.epub-container
    /// conforms to: public.data, public.composite-content
    #[doc(alias = "UTTypeEPUB")]
    #[inline]
    pub fn epub() -> &'static Self {
        unsafe { UTTypeEPUB }
    }

    /// A base type for console logs.
    ///
    /// UTI: public.log
    #[doc(alias = "UTTypeLog")]
    #[inline]
    pub fn log() -> &'static Self {
        unsafe { UTTypeLog }
    }

    /// An Apple Haptics Audio Pattern file.
    ///
    /// UTI: com.apple.haptics.ahap
    #[doc(alias = "UTTypeAHAP")]
    #[inline]
    pub fn ahap() -> &'static Self {
        unsafe { UTTypeAHAP }
    }
}

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {
    static UTTypeFont: &'static ut::Type;
    static UTTypeBookmark: &'static ut::Type;
    static UTTypePKCS12: &'static ut::Type;
    static UTTypeX509Certificate: &'static ut::Type;
    static UTTypeEPUB: &'static ut::Type;
    static UTTypeLog: &'static ut::Type;
    static UTTypeAHAP: &'static ut::Type;
}
