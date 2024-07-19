//
//  vn.h
//  vn
//
//  Created by Yury Korolev on 13.10.2022.
//

#import <Vision/Vision.h>

NS_ASSUME_NONNULL_BEGIN

Class VN_CALCULATE_IMAGE_AESTHETICS_SCORES_REQUEST;
Class VN_DETECT_HORIZON_REQUEST;
Class VN_DETECT_FACE_CAPTURE_QUALITY_REQUEST;
Class VN_RECOGNIZE_ANIMALS_REQUEST;
Class VN_GENERATE_PERSON_SEGMENTAION_REQUEST;
Class VN_CLASSIFY_IMAGE_REQUEST;
Class VN_DETECT_BARCODES_REQUEST;
Class VN_RECOGNIZE_TEXT_REQUEST;
Class VN_GENERATE_OBJECTNESS_BASED_SALIENCY_IMAGE_REQUEST;
Class VN_DETECT_DOCUMENT_SEGMENTATION_REQUEST;
Class VN_GENERATE_ATTENTION_BAED_SALIENCY_IMAGE_REQUEST;
Class VN_GENERATE_IMAGE_FEAUTRE_PRINT_REQUEST;
Class VN_DETECT_FACE_RECTANGLES_REQUEST;
Class VN_IMAGE_REQUEST_HANDLER;
Class VN_SEQUENCE_REQUEST_HANDLER;

__attribute__((constructor))
static void vn_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        VN_CALCULATE_IMAGE_AESTHETICS_SCORES_REQUEST = NSClassFromString(@"VNCalculateImageAestheticsScoresRequest");
        
        VN_DETECT_HORIZON_REQUEST = [VNDetectHorizonRequest class];
        VN_DETECT_FACE_CAPTURE_QUALITY_REQUEST = [VNDetectFaceCaptureQualityRequest class];
        VN_RECOGNIZE_ANIMALS_REQUEST = [VNRecognizeAnimalsRequest class];
        VN_GENERATE_PERSON_SEGMENTAION_REQUEST = [VNGeneratePersonSegmentationRequest class];
        VN_CLASSIFY_IMAGE_REQUEST = [VNClassifyImageRequest class];
        VN_DETECT_BARCODES_REQUEST = [VNDetectBarcodesRequest class];
        VN_RECOGNIZE_TEXT_REQUEST = [VNRecognizeTextRequest class];
        VN_GENERATE_OBJECTNESS_BASED_SALIENCY_IMAGE_REQUEST = [VNGenerateObjectnessBasedSaliencyImageRequest class];
        VN_DETECT_DOCUMENT_SEGMENTATION_REQUEST = [VNDetectDocumentSegmentationRequest class];
        VN_GENERATE_ATTENTION_BAED_SALIENCY_IMAGE_REQUEST = [VNGenerateAttentionBasedSaliencyImageRequest class];
        VN_GENERATE_IMAGE_FEAUTRE_PRINT_REQUEST = [VNGenerateImageFeaturePrintRequest class];
        VN_DETECT_FACE_RECTANGLES_REQUEST = [VNDetectFaceRectanglesRequest class];
        VN_IMAGE_REQUEST_HANDLER = [VNImageRequestHandler class];
        VN_SEQUENCE_REQUEST_HANDLER = [VNSequenceRequestHandler class];
    }
}



NS_ASSUME_NONNULL_END

