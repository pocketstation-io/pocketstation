#include "pocketstation.h"

#include <array>
#include <cstdint>

int main() {
  constexpr uintptr_t kFrameSamples = 960;
  constexpr uintptr_t kOutputBytes = 4'000;
  std::array<float, kFrameSamples> pcm{};
  std::array<unsigned char, kOutputBytes> output{};
  if (pks_opus_max_packet_bytes() != output.size()) {
    return 3;
  }

  PksOpusEncoder *encoder = pks_opus_encoder_create(48'000, 1, 64);
  if (encoder == nullptr) {
    return 1;
  }
  std::array<unsigned char, 1> too_small{0xA5};
  const int rejected =
      pks_encode_opus(encoder, pcm.data(), pcm.size(), too_small.data(),
                      too_small.size());
  if (rejected != PksCodecErrorCode_OutputTooSmall || too_small[0] != 0xA5) {
    pks_opus_encoder_destroy(encoder);
    return 4;
  }
  const int written = pks_encode_opus(encoder, pcm.data(), pcm.size(),
                                     output.data(), output.size());
  pks_opus_encoder_destroy(encoder);
  return written > 0 ? 0 : 2;
}
