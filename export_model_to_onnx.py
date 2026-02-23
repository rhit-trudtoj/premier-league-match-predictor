"""
Export scikit-learn model to ONNX format for Rust inference

This script loads your trained model from the Jupyter notebook and exports it
to ONNX format that can be used by the Rust backend.

Requirements:
    pip install skl2onnx onnx onnxruntime scikit-learn

Usage:
    python export_model_to_onnx.py
"""

import pickle
import numpy as np
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType
import onnx
import onnxruntime as rt

def export_model(model, feature_names, output_path="backend/models/predictor.onnx"):
    """
    Export scikit-learn model to ONNX format

    Args:
        model: Trained scikit-learn model
        feature_names: List of feature names
        output_path: Where to save the ONNX model
    """
    print(f"Exporting model with {len(feature_names)} features...")

    # Define the input type
    # Shape: (batch_size, num_features)
    # None means dynamic batch size
    initial_type = [('input', FloatTensorType([None, len(feature_names)]))]

    try:
        # Convert to ONNX
        onnx_model = convert_sklearn(
            model,
            initial_types=initial_type,
            target_opset=12  # ONNX opset version
        )

        # Save the model
        with open(output_path, "wb") as f:
            f.write(onnx_model.SerializeToString())

        print(f"✓ Model exported successfully to {output_path}")

        # Verify the exported model
        verify_onnx_model(output_path, len(feature_names))

        return True

    except Exception as e:
        print(f"✗ Error exporting model: {e}")
        return False

def verify_onnx_model(model_path, num_features):
    """
    Verify that the ONNX model works correctly
    """
    print("\nVerifying ONNX model...")

    try:
        # Load ONNX model
        onnx_model = onnx.load(model_path)
        onnx.checker.check_model(onnx_model)
        print("✓ ONNX model is valid")

        # Test inference
        session = rt.InferenceSession(model_path)
        input_name = session.get_inputs()[0].name
        output_name = session.get_outputs()[0].name

        # Create dummy input
        dummy_input = np.random.rand(1, num_features).astype(np.float32)

        # Run inference
        result = session.run([output_name], {input_name: dummy_input})

        print(f"✓ Test inference successful")
        print(f"  Input shape: {dummy_input.shape}")
        print(f"  Output shape: {result[0].shape}")
        print(f"  Sample probabilities: {result[0][0]}")

        return True

    except Exception as e:
        print(f"✗ Verification failed: {e}")
        return False

def main():
    """
    Main function - customize this to load your specific model
    """
    print("=" * 60)
    print("Premier League Predictor - Model Export")
    print("=" * 60)

    # TODO: Load your trained model
    # Option 1: If you saved it with pickle
    # try:
    #     with open('model.pkl', 'rb') as f:
    #         model = pickle.load(f)
    # except FileNotFoundError:
    #     print("Error: model.pkl not found. Please save your model first.")
    #     return

    # Option 2: If you need to retrain from the notebook
    # You'll need to extract the model training code from your notebook

    # For demonstration, here's what the feature names should be
    # based on your CLAUDE.md (customize to match your actual features)
    feature_names = [
        'home_avg_xg',
        'away_avg_xg',
        'xg_differential',
        'home_possession',
        'away_possession',
        'possession_differential',
        'home_shots_on_target',
        'away_shots_on_target',
        'home_goals_for',
        'away_goals_for',
        'home_goals_against',
        'away_goals_against',
        'home_form_points',
        'away_form_points',
        'form_differential',
        'head_to_head_ratio',
    ]

    print("\nIMPORTANT: You need to:")
    print("1. Train your model in the Jupyter notebook")
    print("2. Save it using pickle:")
    print("   import pickle")
    print("   with open('model.pkl', 'wb') as f:")
    print("       pickle.dump(model, f)")
    print("3. Update this script to load your model")
    print("4. Run this script again")
    print("\nFeature names expected:")
    for i, name in enumerate(feature_names, 1):
        print(f"  {i:2d}. {name}")

    # Uncomment when you have a model to export
    # export_model(model, feature_names)

if __name__ == "__main__":
    main()
