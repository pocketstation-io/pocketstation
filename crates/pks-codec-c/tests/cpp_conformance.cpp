#include "pks_codec.h"

#include <array>
#include <cstdint>

int main() {
  constexpr uintptr_t kFrameSamples = 960;
  constexpr uintptr_t kOutputBytes = 256;
  std::array<float, kFrameSamples> pcm{};
  std::array<unsigned char, kOutputBytes> output{};

  PksOpusEncoder *encoder = pks_opus_encoder_create(48'000, 1, 64);
  if (encoder == nullptr) {
    return 1;
  }
  const int written = pks_encode_opus(encoder, pcm.data(), pcm.size(),
                                     output.data(), output.size());
  pks_opus_encoder_destroy(encoder);
  return written > 0 ? 0 : 2;
}
